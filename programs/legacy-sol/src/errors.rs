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

  #[msg("No troops on selected tile")]
  NoTroopsOnSelectedTile,

  #[msg("Destination tile is occupied")]
  DestinationOccupied,

  #[msg("Locations from wrong game.")]
  WrongGameLocations,

  #[msg("No troops on target tile.")]
  NoTroopsOnTarget,

  #[msg("Distance exceeds Troop Range")]
  DistanceExceedsTroopRange,
}