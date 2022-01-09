use std::convert::TryInto;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::*;

mod errors;
mod context;
mod account;
mod event;

use account::*;
use errors::*;
use context::*;
use event::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod legacy_sol {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, _bump:u8) -> ProgramResult {
        let admin_acc = &mut ctx.accounts.admin_account;
        admin_acc.key = ctx.accounts.admin.key();
        Ok(())
    }

    pub fn create_game(ctx: Context<InitGame>, id:String, _bump:u8, admin_pk: Pubkey, _0_loc_bump:u8) -> ProgramResult {
        if ctx.accounts.admin_account.key != ctx.accounts.admin.key() {
            return Err(ErrorCode::Unauthorized.into())
        } else {
            let game_account = &mut ctx.accounts.game_account;
            game_account.enabled = true; //TODO: Default to False and then change it via functions. For debug purposes we'll just enable the game
            game_account.authority = admin_pk;
            game_account.id = id.clone();
            //game_account.features = features;
            //game_account.troop_templates = troop_list;
            emit!(NewGame {game_id: id.clone(), game_admin: admin_pk});
            Ok(())
        }
    }

    pub fn init_player(ctx: Context<InitPlayer>, _bump:u8, name: String) -> ProgramResult {
        //Check if the Game is enabled
        if !ctx.accounts.game.enabled {
            return Err(ErrorCode::GameNotEnabled.into())
        } else {
            let player_acc = &mut ctx.accounts.player_account;
            player_acc.authority = ctx.accounts.payer.key();
            player_acc.name = name;
            Ok(())
        }        
    }

    pub fn spawn(ctx: Context<SpawnPlayer>, x:i64, y:i64, _bmp:u8) -> ProgramResult {
        //check that game is enabled
        if !ctx.accounts.game.enabled {
            return Err(ErrorCode::GameNotEnabled.into())
        } else {
            //Check the location is null but has atleast one touching existing location
            let c_loc = &ctx.accounts.connecting_loc;
            if c_loc.x < x-1 || c_loc.x > x+1 || c_loc.y < y-1 || c_loc.y > y+1{
                return Err(ErrorCode::InvalidLocation.into())
            } else {
                //Initialize Location
                let loc = &mut ctx.accounts.location;
                let features = &ctx.accounts.game.features;
                //loc.x = x;
                //loc.y = y; 
                init_loc(loc, features, x, y);
                //Spawn Infantry Unit on starting location
                loc.troops = Some(ctx.accounts.game.troop_templates[0].clone()); //1001 should be Standard Infantry Troop
                //Set Tile Owner to Player Account
                loc.tile_owner = Some(ctx.accounts.player.key());
                loc.game_acc = ctx.accounts.game.key();
                emit!(NewPlayerSpawn {
                    game_acc:ctx.accounts.game.key(), 
                    player: ctx.accounts.player.key(), 
                    coords: Coords {x:x, y:y}
                });
                Ok(())
            }
        }
    }

    pub fn add_features(ctx: Context<ModifyGame>, new_features: Vec<Feature>) -> ProgramResult {
        msg!("Features: {:?}", new_features);
        let game = &mut ctx.accounts.game;
        game.features.extend(new_features.iter().cloned());
        Ok(())
    }


    pub fn add_troop_templates(ctx: Context<ModifyGame>, new_troops: Vec<Troop>) -> ProgramResult {
        msg!("Troop Templates: {:?}", new_troops);
        let game = &mut ctx.accounts.game;
        game.troop_templates.extend(new_troops.iter().cloned());  
        Ok(())
    }

    pub fn initalize_location(ctx: Context<InitLoc>, x:i64, y:i64, _bmp:u8) -> ProgramResult {
        //check that game is enabled
        if !ctx.accounts.game.enabled {
            return Err(ErrorCode::GameNotEnabled.into())
        } 

        let loc = &mut ctx.accounts.location;
        let c_loc = &ctx.accounts.connecting_loc;
        if c_loc.x < x-1 || c_loc.x > x+1 || c_loc.y < y-1 || c_loc.y > y+1{
            return Err(ErrorCode::InvalidLocation.into())
        } 

        init_loc(loc, &ctx.accounts.game.features, x, y);
        loc.game_acc = ctx.accounts.game.key();
        emit!(NewLocationInitalized {
            game_acc: ctx.accounts.game.key(),
            coords: Coords {x: x, y: y},
            feature:loc.feature.as_ref().unwrap().clone()});
        Ok(())
    }

    pub fn move_troops(ctx: Context<MoveOrAttack>) -> ProgramResult {
        //check that game is enabled
        let game = &ctx.accounts.game;
        if !game.enabled {
            return Err(ErrorCode::GameNotEnabled.into())
        } 

        //Check that the tiles are connecting
        let dest = &mut ctx.accounts.destination;
        let from = &mut ctx.accounts.from;

        if from.game_acc != game.key() || dest.game_acc != game.key() {
            return Err(ErrorCode::WrongGameLocations.into())
        }


        if from.x < dest.x-1 || from.x > dest.x+1 || from.y < dest.y-1 || from.y > dest.y+1{
            return Err(ErrorCode::InvalidLocation.into())
        } 

        //Check that the from tile has troops
        if from.troops == None {
            return Err(ErrorCode::NoTroopsOnSelectedTile.into());
        }

        let player = &ctx.accounts.player;
        //Check it belongs to the player
        if player.key() != from.tile_owner.unwrap() {
            return Err(ErrorCode::PlayerLacksOwnership.into());
        }
        //Check that the to tile does not have troops
        if dest.troops != None {
            return Err(ErrorCode::DestinationOccupied.into());
        }
        //Move the troops
        dest.troops = from.troops.clone();
        from.troops = None;
        dest.tile_owner = Some(player.key());
        from.tile_owner = None;

        emit! (TroopsMoved {
            game_acc: game.key(),
            from: Coords {x:from.x, y:from.y},
            dest: Coords {x:dest.x, y:dest.y},
            moving_player_acc: player.key(),
            moving_troops: dest.troops.as_ref().unwrap().clone()
        });
        Ok(())
    }

    pub fn attack(ctx: Context<MoveOrAttack>) -> ProgramResult {
        //check that game is enabled
        let game = &ctx.accounts.game;
        let player = &ctx.accounts.player;
        if !game.enabled {
            return Err(ErrorCode::GameNotEnabled.into())
        } 

        //Check that the tiles are connecting
        let dest = &mut ctx.accounts.destination;
        let from = &mut ctx.accounts.from;

        if from.game_acc != game.key() || dest.game_acc != game.key() {
            return Err(ErrorCode::WrongGameLocations.into())
        }
        
        if from.tile_owner != Some(player.key()) {
            return Err(ErrorCode::PlayerLacksOwnership.into())
        }

        if dest.tile_owner == Some(player.key()) {
            return Err(ErrorCode::NoFriendlyFire.into())
        }

        if from.troops == None {
            return Err(ErrorCode::NoTroopsOnSelectedTile.into())
        }

        if dest.troops == None {
            return Err(ErrorCode::NoTroopsOnTarget.into())
        }

        //Check the distance between the two locations is less than or equal to the range of the attacking troops
        let distance:f64 = (((dest.x - from.x).pow(2) + (dest.y - from.y).pow(2)) as f64).sqrt();
        let unit_range = from.troops.as_ref().unwrap().range;
        if distance > unit_range.into() {
            return Err(ErrorCode::DistanceExceedsTroopRange.into())
        }
        let mut atk_troops = from.troops.as_ref().unwrap().clone();
        let mut def_troops = dest.troops.as_ref().unwrap().clone();

        if atk_troops.range == 1 {
            let atk_atk = get_atk(&atk_troops, &def_troops, 0);
            let def_atk = get_atk(&def_troops, &atk_troops, 1);
            if def_troops.power.checked_sub(atk_atk) == None {
                //atk troops wiped out
                dest.troops = None;
                dest.tile_owner = None;
            } else {
                def_troops.power -= atk_atk;
                dest.troops = Some(def_troops);
            }
            if atk_troops.power.checked_sub(def_atk) == None {
                //atk troops wiped out
                from.troops = None;
                from.tile_owner = None;
            } else {
                atk_troops.power -= def_atk;
                from.troops = Some(atk_troops);
            }

            emit!(Combat {
                game_acc: game.key(),
                from: Coords {x:from.x, y:from.y},
                dest: Coords {x:dest.x, y:dest.y},
                atk_dmg: atk_atk,
                def_dmg: def_atk
            });

        } else {
            let atk_atk = get_atk(&atk_troops, &def_troops, 0);
            if def_troops.power.checked_sub(atk_atk) == None {
                //atk troops wiped out
                dest.troops = None;
                dest.tile_owner = None;
            } else {
                def_troops.power -= atk_atk;
                dest.troops = Some(def_troops);
            }
            emit!(Combat {
                game_acc: game.key(),
                from: Coords {x:from.x, y:from.y},
                dest: Coords {x:dest.x, y:dest.y},
                atk_dmg: atk_atk,
                def_dmg: 0
            });
        }

        Ok(())
    }

    pub fn debug(_ctx: Context<Debug>, x:Vec<DebugStruct>) -> ProgramResult {
        msg!("X: {:?}", x);
        Ok(())
    }   
}


pub fn get_atk(attacking: &Troop, defending: &Troop, idx:usize) -> u8{
    //returns a random number between 0 to power
    let mut attacking_power = get_random_u8(idx) / (255/attacking.power);
    if defending.class == TroopClass::Infantry {
        attacking_power = attacking_power.saturating_add(attacking.mod_inf.try_into().unwrap());
    } else if defending.class == TroopClass::Armor {
        attacking_power = attacking_power.saturating_add(attacking.mod_armor.try_into().unwrap());
    } else if defending.class == TroopClass::Aircraft {
        attacking_power = attacking_power.saturating_add(attacking.mod_air.try_into().unwrap());
    } 

    return attacking_power;
}

/*
 * Spawns a random feature on the location
*/
pub fn init_loc(loc:&mut Account<Location>, features:&Vec<Feature>, x:i64, y:i64) {
    loc.x = x;
    loc.y = y;
    //loc.feature = Some(features[usize::from(get_random_u8())].clone());
    //loc.feature = features.get(&get_random_u8()).cloned();
    let random_number = get_random_u8(0);
    //msg!("Random Feature Num: {}", random_number);
    for feature in features{
        if random_number < feature.weight {
            loc.feature = Some(feature.clone());
            break;
        }
    }
}

/**
 * Generates a random number using the slothash[0]
 * Idx determines where from the slot hash it pulls the random number from
 * Useful when multiple random numbers are required, such as in 2 way combat
 */

pub fn get_random_u8(idx:usize) -> u8 {
    let clock = Clock::get().unwrap();
    let num = &hash(&clock.slot.to_be_bytes()).to_bytes()[idx];
    return *num;
}



/*
//use std::convert::TryInto;

msg!("{} is current Timestamp", clock.unix_timestamp);
let clock = Clock::get().unwrap();
msg!("{} is current Slot", clock.slot);
msg!("{:?} is the hash of the slot", hash(&clock.slot.to_be_bytes()));
let slc = &hash(&clock.slot.to_be_bytes()).to_bytes()[0 .. 8];
let num: u64 = u64::from_be_bytes(slc.try_into().unwrap());
msg!("{:?} is the random value", num);
*/

/* COMBAT
fn main() {
  let r:u8 = 56;
  let max:u8 = 255;
  let p:u8 = 99;
  let s:u8 = max/p;
  println!("Segment: {}", s);
  let target = r/s;
  println!("Target: {}", target);
}
*/