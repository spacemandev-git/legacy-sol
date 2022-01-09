use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
  #[msg("You are not authorized to perform this action.")]
  Unauthorized,

  #[msg("Game is not enabled.")]
  GameNotEnabled,

  #[msg("Invalid Location")]
  InvalidLocation,

  #[msg("Player doesn't own the tile")]
  PlayerLacksOwnership,

  #[msg("No troops on tile to move")]
  NoTroopsToMove,

  #[msg("Destination tile is occupied")]
  DestinationOccupied

}