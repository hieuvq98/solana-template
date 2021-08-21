use anchor_lang::prelude::*;
use anchor_lang::solana_program;

#[program]
mod coin98_lunapad {
  use super::*;
}

#[error]
pub enum ErrorCode {

  #[msg("Coin98Lunapad: Not an owner.")]
  InvalidOwner,

  #[msg("Coin98Lunapad: Not new owner.")]
  InvalidNewOwner,

  #[msg("Coin98Lunapad: Transaction failed.")]
  TransactionFailed,
}
