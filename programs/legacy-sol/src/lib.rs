use anchor_lang::prelude::*;
//use std::collections::HashMap;
mod errors;

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
            return Err(errors::NotAuthorizedError::Unauthorized.into())
        } else {
            let game_account = &mut ctx.accounts.game_account;
            game_account.enabled = true; //TODO: Default to False and then change it via functions. For debug purposes we'll just enable the game
            game_account.admin = admin_pk;
            game_account.id = id;
            Ok(())
        }
    }
}

#[derive(Accounts)]
#[instruction(_bump: u8)]
pub struct Initialize<'info> {
    #[account(init, seeds=[admin.key().as_ref()], bump=_bump, payer=admin, space=8+32)]
    pub admin_account: Account<'info, AdminAccount>,
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct AdminAccount {
    key: Pubkey,
}

#[derive(Accounts)]
#[instruction(id: String, _bump:u8, admin_pk: Pubkey)]
pub struct InitGame<'info> {
    pub admin_account: Account<'info, AdminAccount>,
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(init, seeds=[id.as_ref()], bump=_bump, payer=admin, space=8+32+58+1)]
    pub game_account: Account<'info, GameAccount>,
}

#[account]
#[derive(Default)]
pub struct GameAccount {
    id: String,
    admin: Pubkey,
    enabled: bool,
}