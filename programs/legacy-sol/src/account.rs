use anchor_lang::prelude::*;

#[account]
pub struct Admin {
    pub key: Pubkey,
}

#[account]
pub struct Game {
    pub id: String,
    pub admin: Pubkey,
    pub enabled: bool,
}

#[account]
pub struct Player{
    pub name: String,
    pub authority: Pubkey,
    pub troop_cards: [u32; 10],
}

#[account]
pub struct Location{ 
    pub x: i64,
    pub y: i64,
    pub feature: Option<Feature>,
    pub troops: Option<Troop>,
    pub tile_owner: Option<Pubkey>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Feature {
    name: String,
    rarity: u8,
    next_scan: i64, 
}

// total space: 64+32+4 = 100 (128)
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Troop{
    pub id: String, //64  example is 63: //https://arweave.net/zt3-t8SHDSck0TLcSuC-hdQb2E0civ0DVMRgwf6sCz0
    pub class: TroopClass, //4
    pub power: u8, //8
    pub mod_inf: i8, //8
    pub mod_armor: i8, //8
    pub mod_air: i8, //8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum TroopClass {
    Infantry,
    Armor,
    Aircraft
}

impl Default for TroopClass {
    fn default() -> Self { TroopClass::Infantry }
}