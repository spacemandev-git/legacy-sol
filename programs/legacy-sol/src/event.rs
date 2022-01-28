use anchor_lang::prelude::*;
use crate::account::*;

#[event]
pub struct NewGame {
  pub game_id: String,
  pub game_admin: Pubkey
}

#[event]
pub struct NewLocationInitalized {
  pub game_acc: Pubkey,
  pub coords: Coords,
  pub feature: Feature
}

#[event]
pub struct TroopsMoved {
  pub game_acc: Pubkey,
  pub from: Coords,
  pub dest: Coords,
  pub moving_player_acc: Pubkey,
  pub moving_troops: Troop
}

#[event]
pub struct TroopsDeath {
  pub game_acc: Pubkey,
  pub dead_troop_player_acc: Pubkey,
  pub killer_troop_player_acc: Pubkey,
  pub dead_troops: Troop,
  pub killer_troops: Troop,
  pub troop_tile: Coords
}

#[event]
pub struct Combat {
  pub game_acc: Pubkey,
  pub from: Coords,
  pub dest: Coords,
  pub atk_dmg: u8,
  pub def_dmg: u8
}

#[event]
pub struct CardRedeemed {
  pub game_acc: Pubkey,
  pub player: Pubkey,
  pub card_redeemed: Card,
}

#[event]
pub struct UnitModded {
  pub game_acc: Pubkey,
  pub player: Pubkey,
  pub unit_mod: UnitMod,
}

#[event]
pub struct UnitSpawned {
  pub game_acc: Pubkey,
  pub player: Pubkey,
  pub unit: Troop,
}