use anchor_lang::prelude::*;


#[account]
pub struct Admin {
    pub key: Pubkey,
}

#[account]
#[derive(Default)]
pub struct Game {
    pub id: String,
    pub admin: Pubkey,
    pub enabled: bool,
}

#[account]
pub struct Player{
    
}