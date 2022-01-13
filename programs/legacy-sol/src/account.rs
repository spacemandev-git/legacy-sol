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
    pub locations: Vec<Coords>,
    pub new_player_unit: Troop,
    pub decks: DeckLen,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct DeckLen{
    pub basic: u64,
    pub rare: u64,
    pub legendary: u64
}

#[account]
pub struct Player{
    pub name: String,
    pub authority: Pubkey,
    pub cards: Vec<Card>,
    pub redeemable_cards: Vec<RedeemableCard>
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct RedeemableCard{
    pub drop_table: DropTable,
    pub id: u64,
}

#[account]
pub struct Location{ 
    pub game_acc: Pubkey,
    pub coords: Coords,
    pub feature: Option<Feature>,
    pub troops: Option<Troop>,
    pub tile_owner: Option<Pubkey>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Feature {
    pub drop_table: Option<DropTable>,
    pub weight: u8, 
    pub name: String,
    pub scan_recovery: u64,
    pub last_scanned: u64, // slot when this feature was last scanned
    pub times_scanned: u64
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum DropTable {
    None,
    Basic,
    Rare,
    Legendary
}

impl Default for DropTable {
    fn default() -> Self {DropTable::None}
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
    pub recovery: u16, //might have *really* slow, really powerful units in the future?
    pub last_moved: u64
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug, Copy)]
pub enum TroopClass {
    Infantry,
    Armor,
    Aircraft
}

impl Default for TroopClass {
    fn default() -> Self { TroopClass::Infantry }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub struct UnitMod {
    pub name: String,
    pub link: String,
    pub class: Option<TroopClass>,
    pub range: i8,
    pub power: i8,
    pub mod_inf: i8,
    pub mod_armor: i8,
    pub mod_air: i8,
    pub recovery: i8
}

#[account]
pub struct CardTemplate{
    pub card: Card
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub struct Card {
    pub drop_table: DropTable,
    pub id: u64,
    pub card_type: CardType,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum CardType {
    None, 
    Unit { 
        unit: Troop
    },
    UnitMod {
        umod: UnitMod
    },
}

impl Default for CardType {
    fn default() -> Self { CardType::None} 
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub struct Coords{
  pub x:i8,
  pub y:i8
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct DebugStruct {
    pub x: u8,
    pub y: i64, 
}
