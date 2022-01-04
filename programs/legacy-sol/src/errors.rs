use anchor_lang::prelude::*;

#[error]
pub enum NotAuthorizedError {
  #[msg("You are not authorized to perform this action.")]
  Unauthorized,
}