use anchor_lang::prelude::*;


#[account]
pub struct AdminAccount {
    key: Pubkey,
}

#[account]
#[derive(Default)]
pub struct GameAccount {
    id: String,
    admin: Pubkey,
    enabled: bool,
}