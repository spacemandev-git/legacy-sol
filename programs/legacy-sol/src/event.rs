use anchor_lang::prelude::*;
use crate::account::*;

#[event]
pub struct NewGame {
  pub game_id: String,
  pub game_admin: Pubkey
}

#[event]
pub struct NewPlayerSpawn {
  pub game_acc: Pubkey,
  pub player: Pubkey,
  pub coords: Coords
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
pub struct Combat {
  pub game_acc: Pubkey,
  pub from: Coords,
  pub dest: Coords,
  pub atk_dmg: u8,
  pub def_dmg: u8
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub struct Coords{
  pub x:i64,
  pub y:i64
}