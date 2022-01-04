use anchor_lang::prelude::*;
//use std::collections::HashMap;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod legacy_sol {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, bump:u8) -> ProgramResult {
        let admin_acc = &mut ctx.accounts.admin_account;
        admin_acc.admin = ctx.accounts.admin.key();
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Initialize<'info> {
    #[account(init, seeds=[admin.key().as_ref()], bump=bump, payer=admin, space=8+32)]
    pub admin_account: Account<'info, AdminAccount>,
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct AdminAccount {
    admin: Pubkey,
}

/*
#[account]
pub struct GameAccount<'info> {
    id: String,
    admin: Signer<'info>,
    enabled: bool,
}

#[account]
#[derive(Default)]
pub struct Map {
    coords: HashMap<(i64,i64), u64>,
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}
*/