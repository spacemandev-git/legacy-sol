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

  #[msg("No friendly fire allowed")]
  NoFriendlyFire,

  #[msg("Scan waiting to be redeemed!")]
  RedemptionAvailable,

  #[msg("Location is still in cooldown from last scan")]
  LocationOnCooldown,

  #[msg("Invalid Card Passed In")]
  InvalidCard,

  #[msg("Mod cannot be applied to Unit.")]
  InvalidMod,

  #[msg("Feature can't be scanned")]
  FeatureNotScannable,

  #[msg("Unit Mod doesn't match the Unit's class")]
  InvalidUnitMod,

  #[msg("Unit is still recovering")]
  UnitRecovering,
}