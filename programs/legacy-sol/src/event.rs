use anchor_lang::prelude::*;

#[event]
pub struct EventNewGame {
  pub game_id: String,
  pub game_admin: Pubkey
}