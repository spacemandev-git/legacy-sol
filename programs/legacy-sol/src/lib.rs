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

declare_id!("Cz4TVYSDxwobuiKdtZY8ejp3hWL7WfCbPNYGUqnNBVSe");

#[program]
pub mod legacy_sol {
    use super::*;

    pub fn create_game(ctx: Context<InitGame>, id:String, _bump:u8, _0_loc_bump:u8, starting_card:Card) -> ProgramResult {
        let game_account = &mut ctx.accounts.game_account;
        game_account.enabled = true; //TODO: Default to False and then change it via functions. For debug purposes we'll just enable the game
        game_account.authority = ctx.accounts.authority.key();
        game_account.id = id.clone();
        game_account.starting_card = starting_card;
        //game_account.features = features;
        //game_account.troop_templates = troop_list;
        
        let start_loc = &mut ctx.accounts.start_location;
        //give it a feature
        start_loc.game_acc = game_account.key();
        start_loc.coords = Coords {x:0, y:0};
        start_loc.feature = Some(Feature {
            drop_table: Some(DropTable::None),
            link: "none.png".to_string(),
            weight: 100,
            name: "blank_space".to_string(),
            scan_recovery: 0,
            last_scanned: 0,
            times_scanned: 0
        });
        start_loc.troops = None;
        start_loc.tile_owner = None;
        //add it to game locs 
        game_account.locations.push(Coords {x:0, y:0});


        emit!(NewGame {game_id: id.clone(), game_admin: ctx.accounts.authority.key()});
        Ok(())
    
    }

    pub fn init_player(ctx: Context<InitPlayer>, _bump:u8, name: String) -> ProgramResult {
        //Check if the Game is enabled
        if !ctx.accounts.game.enabled {
            return Err(ErrorCode::GameNotEnabled.into())
        } else {
            let player_acc = &mut ctx.accounts.player_account;
            player_acc.authority = ctx.accounts.player.key();
            player_acc.name = name;
            player_acc.cards = vec![ctx.accounts.game.starting_card.clone()];
            player_acc.redeemable_cards = vec![];
            Ok(())
        }        
    }

    pub fn add_features(ctx: Context<ModifyGame>, new_features: Vec<Feature>) -> ProgramResult {
        let game = &mut ctx.accounts.game;
        game.features.extend(new_features.iter().cloned());
        Ok(())
    }

    pub fn init_card(ctx: Context<InitCard>, card: Card, _bmp:u8) -> ProgramResult {
        ctx.accounts.card_acc.card = card.clone();
        match card.drop_table {
            DropTable::Basic => ctx.accounts.game.decks.basic += 1,
            DropTable::Rare => ctx.accounts.game.decks.rare += 1,
            DropTable::Legendary => ctx.accounts.game.decks.legendary += 1,
            _ => {}
        }
        Ok(())
    }

    pub fn init_location(ctx: Context<InitLoc>, x:i8, y:i8, _bmp:u8) -> ProgramResult {
        //check that game is enabled
        let game = &mut ctx.accounts.game;
        if !game.enabled {
            return Err(ErrorCode::GameNotEnabled.into())
        } 

        let loc = &mut ctx.accounts.location;
        let c_loc = &ctx.accounts.connecting_loc.coords;
        if c_loc.x < x-1 || c_loc.x > x+1 || c_loc.y < y-1 || c_loc.y > y+1{
            return Err(ErrorCode::InvalidLocation.into())
        } 

        init_loc(loc, &game.features, x, y);
        loc.game_acc = game.key();
        game.locations.push(Coords {x, y});
        emit!(NewLocationInitalized {
            game_acc: game.key(),
            coords: Coords {x: x, y: y},
            feature:loc.feature.as_ref().unwrap().clone()});
        Ok(())
    }

    pub fn scan(ctx:Context<Scan>) -> ProgramResult {
        let game = &ctx.accounts.game;
        let location = &mut ctx.accounts.location;
        let player = &mut ctx.accounts.player;
        //check game is enabled
        if !game.enabled {
            return Err(ErrorCode::GameNotEnabled.into())
        }
        //check location belongs to game
        if location.game_acc != game.key() {
            return Err(ErrorCode::InvalidLocation.into())
        }
        //check the player owns the tile
        if location.tile_owner != Some(player.key()) {
            return Err(ErrorCode::PlayerLacksOwnership.into())
        }        

        let mut feature = location.feature.as_ref().unwrap().clone();
        if feature.drop_table == None {
            return Err(ErrorCode::FeatureNotScannable.into());
        }

        //check current slot > last_scanned+feature_scan_delay OR last scan == 0
        let next_scan = feature.last_scanned + feature.scan_recovery;
        let clock = Clock::get().unwrap();

        if next_scan < clock.slot {
            return Err(ErrorCode::LocationOnCooldown.into())
        }

        //give the player cards
        match feature.drop_table {
            Some(DropTable::Basic) => player.redeemable_cards.push(RedeemableCard {
                drop_table: DropTable::Basic,
                id: get_random_u64(game.decks.basic)
            }),
            Some(DropTable::Rare) => player.redeemable_cards.push(RedeemableCard {
                drop_table: DropTable::Rare,
                id: get_random_u64(game.decks.rare)
            }), 
            Some(DropTable::Legendary) => player.redeemable_cards.push(RedeemableCard {
                drop_table: DropTable::Legendary,
                id: get_random_u64(game.decks.legendary)
            }),
            _ => {}
        }


        //set the feature's last scan to now, to be redeemed when delay has passed
        feature.last_scanned = clock.slot;
        feature.times_scanned += 1;
        location.feature = Some(feature);
        
        Ok(())
    }

    pub fn redeem(ctx:Context<Redeem>) -> ProgramResult {
        let player = &mut ctx.accounts.player;
        let card = &ctx.accounts.card;
        let redeemable_card = player.redeemable_cards.pop().unwrap();
        if card.card.id == redeemable_card.id && redeemable_card.drop_table == card.card.drop_table {
            player.cards.push(card.card.clone());
        } else {
            return Err(ErrorCode::InvalidCard.into())
        }
        Ok(())
    }

    pub fn play_card(ctx:Context<PlayCard>, idx: u16) -> ProgramResult {
        let game = &ctx.accounts.game;
        let player = &mut ctx.accounts.player;
        let location = &mut ctx.accounts.location;

        let card_idx = usize::from(idx);
        if !game.enabled {
            return Err(ErrorCode::GameNotEnabled.into())
        } 
        if location.game_acc != game.key() {
            return Err(ErrorCode::WrongGameLocations.into())
        }

        let card: Card = player.cards.remove(card_idx);
        match card.card_type {
            CardType::Unit {unit} => {
                //check if location is empty
                if location.troops != None {
                    return Err(ErrorCode::DestinationOccupied.into())
                }
                //spawn in the troop
                location.troops = Some(unit.clone());
                //set location owner to player
                location.tile_owner = Some(player.key());
            },
            CardType::UnitMod {
                umod
            } => {
                //check location has troops
                if location.troops == None {
                    return Err(ErrorCode::NoTroopsOnSelectedTile.into())
                }
                //check the tile owner is player
                if location.tile_owner != Some(player.key()){
                    return Err(ErrorCode::PlayerLacksOwnership.into())
                }
                //modify troop based on UnitMod
                let mut modified_troops = location.troops.as_ref().unwrap().clone();
                
                //check that the UnitMod class is None or matches troops
                if umod.class != Some(modified_troops.class) || umod.class != None {
                    return Err(ErrorCode::InvalidUnitMod.into());
                }


                if umod.range < 0 {
                    modified_troops.range = modified_troops.range.saturating_sub(umod.range.abs().try_into().unwrap());
                } else {
                    modified_troops.range = modified_troops.range.saturating_add(umod.range.abs().try_into().unwrap());
                }
                if modified_troops.range < 1 {
                    return Err(ErrorCode::InvalidMod.into())
                }

                if umod.power < 0 {
                    modified_troops.power = modified_troops.power.saturating_sub(umod.power.abs().try_into().unwrap());
                } else {
                    modified_troops.power = modified_troops.power.saturating_add(umod.power.abs().try_into().unwrap());
                }
                if modified_troops.power < 1 {
                    return Err(ErrorCode::InvalidMod.into())
                }

                if umod.recovery < 0 {
                    modified_troops.recovery = modified_troops.recovery.saturating_sub(umod.recovery.abs().try_into().unwrap());
                } else {
                    modified_troops.recovery = modified_troops.recovery.saturating_add(umod.recovery.abs().try_into().unwrap());
                }

                modified_troops.mod_inf = modified_troops.mod_inf.saturating_add(umod.mod_inf);
                modified_troops.mod_armor = modified_troops.mod_armor.saturating_add(umod.mod_armor);
                modified_troops.mod_air = modified_troops.mod_air.saturating_add(umod.mod_air);
                location.troops = Some(modified_troops);
            },
            CardType::None => {}
        }

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


        if from.coords.x < dest.coords.x-1 || from.coords.x > dest.coords.x+1 || from.coords.y < dest.coords.y-1 || from.coords.y > dest.coords.y+1{
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

        let mut from_troops = from.troops.as_ref().unwrap().clone();
        let next_valid_move_slot:u64 = from_troops.last_moved.saturating_add(from_troops.recovery.into());
        let clock = Clock::get().unwrap();
        if clock.slot < next_valid_move_slot {
            return Err(ErrorCode::UnitRecovering.into())
        }


        //Move the troops
        from_troops.last_moved = clock.slot;
        dest.troops = Some(from_troops);
        from.troops = None;
        dest.tile_owner = Some(player.key());
        from.tile_owner = None;

        emit! (TroopsMoved {
            game_acc: game.key(),
            from: Coords {x:from.coords.x, y:from.coords.y},
            dest: Coords {x:dest.coords.x, y:dest.coords.y},
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
        
        let mut from_troops = from.troops.as_ref().unwrap().clone();
        let next_valid_move_slot:u64 = from_troops.last_moved.saturating_add(from_troops.recovery.into());
        let clock = Clock::get().unwrap();
        if clock.slot < next_valid_move_slot {
            return Err(ErrorCode::UnitRecovering.into())
        }
        from_troops.last_moved = clock.slot;
        //Check the distance between the two locations is less than or equal to the range of the attacking troops
        let distance:f64 = (((dest.coords.x - from.coords.x).pow(2) + (dest.coords.y - from.coords.y).pow(2)) as f64).sqrt();
        let unit_range = from.troops.as_ref().unwrap().range;
        if distance > unit_range.into() {
            return Err(ErrorCode::DistanceExceedsTroopRange.into())
        }
        let mut atk_troops = from_troops;
        let mut def_troops = dest.troops.as_ref().unwrap().clone();

        if atk_troops.range == 1 {
            let atk_atk = get_atk(&atk_troops, &def_troops, 0);
            //msg!("Atk Dmg: {}", atk_atk);
            let def_atk = get_atk(&def_troops, &atk_troops, 1);
            //msg!("Def Dmg: {}", def_atk);

            if def_troops.power.checked_sub(atk_atk) == Some(0) || 
               def_troops.power.checked_sub(atk_atk) == None {
                //def troops wiped out
                dest.troops = None;
                dest.tile_owner = None;
            } else {
                def_troops.power -= atk_atk;
                dest.troops = Some(def_troops);
            }
            if atk_troops.power.checked_sub(def_atk) == Some(0) ||
               atk_troops.power.checked_sub(def_atk) == None {
                //atk troops wiped out
                from.troops = None;
                from.tile_owner = None;
            } else {
                atk_troops.power -= def_atk;
                from.troops = Some(atk_troops);
            }

            emit!(Combat {
                game_acc: game.key(),
                from: Coords {x:from.coords.x, y:from.coords.y},
                dest: Coords {x:dest.coords.x, y:dest.coords.y},
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
                from: Coords {x:from.coords.x, y:from.coords.y},
                dest: Coords {x:dest.coords.x, y:dest.coords.y},
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
    let mut attacking_power = get_random_u8(idx) / (u8::MAX/attacking.power);
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
pub fn init_loc(loc:&mut Account<Location>, features:&Vec<Feature>, x:i8, y:i8) {
    loc.coords.x = x;
    loc.coords.y = y;
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

/**
 * Returns a random number below the Max
 * Can replace random u8 function BUT u8 function is useful for getting multiple random numbers in a slot, which this can't do.
 */ 
pub fn get_random_u64(max: u64) -> u64 {
    let clock = Clock::get().unwrap();
    let slice = &hash(&clock.slot.to_be_bytes()).to_bytes()[0..8];
    let num: u64 = u64::from_be_bytes(slice.try_into().unwrap());
    let target = num/(u64::MAX/max);
    return target;
}