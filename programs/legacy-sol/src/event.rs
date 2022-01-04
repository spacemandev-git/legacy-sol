use anchor_lang::prelude::*;

#[event]
pub struct Event_New_Game {
  pub game_id: String,
  pub game_admin: Pubkey
}