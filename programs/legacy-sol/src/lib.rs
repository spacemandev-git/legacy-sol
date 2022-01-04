use anchor_lang::prelude::*;
//use std::collections::HashMap;
mod errors::ErrorCode;
mod context::*;
mod account::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod legacy_sol {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, _bump:u8) -> ProgramResult {
        let admin_acc = &mut ctx.accounts.admin_account;
        admin_acc.key = ctx.accounts.admin.key();
        Ok(())
    }

    pub fn create_game(ctx: Context<InitGame>, id:String, _bump:u8, admin_pk: Pubkey) -> ProgramResult {
        if ctx.accounts.admin.key() != ctx.accounts.admin_account.key {
            return Err(ErrorCode::Unauthorized.into())
        } else {
            let game_account = &mut ctx.accounts.game_account;
            game_account.enabled = true; //TODO: Default to False and then change it via functions. For debug purposes we'll just enable the game
            game_account.admin = admin_pk;
            game_account.id = id;
            Ok(())
        }
    }
}