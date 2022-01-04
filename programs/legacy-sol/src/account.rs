use anchor_lang::prelude::*;


#[account]
pub struct AdminAccount {
    pub key: Pubkey,
}

#[account]
#[derive(Default)]
pub struct GameAccount {
    pub id: String,
    pub admin: Pubkey,
    pub enabled: bool,
}