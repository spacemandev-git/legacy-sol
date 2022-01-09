use anchor_lang::prelude::*;
use crate::account::*;

#[event]
pub struct NewGame {
  pub game_id: String,
  pub game_admin: Pubkey
}

#[event]
pub struct NewPlayerSpawn {
  pub player: Pubkey,
  pub x: i64,
  pub y: i64
}

#[event]
pub struct NewLocationInitalized {
  pub x: i64,
  pub y: i64,
  pub feature: Feature
}


#[event]
pub struct TroopsMoved {
  pub from: Pubkey,
  pub dest: Pubkey,
  pub moving_player_acc: Pubkey,
  pub moving_troops: Troop
}