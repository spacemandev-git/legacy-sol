use anchor_lang::prelude::*;

#[account]
pub struct Admin {
    pub key: Pubkey,
}

#[account]
pub struct Game {
    pub id: String,
    pub authority: Pubkey,
    pub enabled: bool,
    pub features: Vec<Feature>,
    pub new_player_unit: Troop,
    pub deck_len: u64,
    pub feature_scan_delay: u64 //How long features must go before they can be scanned again
}

#[account]
pub struct Player{
    pub name: String,
    pub authority: Pubkey,
    pub cards: Vec<Card>,
    pub redeemable_cards: Vec<u64>
}

#[account]
pub struct Location{ 
    pub game_acc: Pubkey,
    pub x: i64,
    pub y: i64,
    pub feature: Option<Feature>,
    pub troops: Option<Troop>,
    pub tile_owner: Option<Pubkey>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Feature {
    pub weight: u8,
    pub name: String,
    pub last_scanned: u64, // slot when this feature was last scanned
    pub times_scanned: u64
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub struct Troop{
    pub name: String, //64
    pub link: String, //64  example is 63: //https://arweave.net/zt3-t8SHDSck0TLcSuC-hdQb2E0civ0DVMRgwf6sCz0
    pub class: TroopClass, 
    pub range: u8,
    pub power: u8, 
    pub mod_inf: i8, 
    pub mod_armor: i8,
    pub mod_air: i8, 
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum TroopClass {
    Infantry,
    Armor,
    Aircraft
}

impl Default for TroopClass {
    fn default() -> Self { TroopClass::Infantry }
}

#[account]
pub struct CardTemplate{
    pub card: Card
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub struct Card {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub link: String, //link to image
    pub card_type: CardType,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum CardType {
    None,
    Unit { 
        unit: Troop
    },
    UnitMod {
        range: i8,
        power: i8,
        mod_inf: i8,
        mod_armor: i8,
        mod_air: i8
    },
}

impl Default for CardType {
    fn default() -> Self { CardType::None }
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct DebugStruct {
    pub x: u8,
    pub y: i64, 
}
