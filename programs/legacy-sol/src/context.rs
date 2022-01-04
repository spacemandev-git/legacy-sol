use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(_bump: u8)]
pub struct Initialize<'info> {
    #[account(init, seeds=[admin.key().as_ref()], bump=_bump, payer=admin, space=8+32)]
    pub admin_account: Account<'info, AdminAccount>,
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>
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