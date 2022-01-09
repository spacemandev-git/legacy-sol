use anchor_lang::prelude::*;

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
  pub y: i64
}


