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
            game_account.admin = admin_pk;
            game_account.id = id.clone();
            //game_account.features = features;
            //game_account.troop_templates = troops;
            emit!(EventNewGame {game_id: id.clone(), game_admin: admin_pk});
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
                //loc.troops = ctx.accounts.game.troop_templates.get(&1001).cloned(); //1001 should be Standard Infantry Troop
                //Set Tile Owner to Player Account
                loc.tile_owner = Some(ctx.accounts.player.key());
                Ok(())
            }
        }
    }
}


pub fn init_loc(loc:&mut Account<Location>, features:&Vec<Option<Feature>>, x:i64, y:i64) {
    loc.x = x;
    loc.y = y;
    loc.feature = features[usize::from(get_random_u8())].clone();
    
    //loc.feature = features.get(&get_random_u8()).cloned();
}

pub fn get_random_u8() -> u8 {
    let clock = Clock::get().unwrap();
    let num = &hash(&clock.slot.to_be_bytes()).to_bytes()[0];
    return *num;
}

/*
pub fn init_loc(loc: Account, x:i64, y:i64){
}

pub fn get_random_u8() -> u8 {
    return 5;
}

*/

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