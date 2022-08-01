use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {

  #[msg("Coin98Starship: Forbidden.")]
  Blacklisted,

  #[msg("Coin98Starship: Time must be set in the future.")]
  FutureTimeRequired,

  #[msg("Coin98Starship: Invalid account.")]
  InvalidAccount,

  #[msg("Coin98Starship: Not an owner.")]
  InvalidOwner,

  #[msg("Coin98Starship: Invalid registration time range.")]
  InvalidRegistrationTime,

  #[msg("Coin98Starship: Invalid sale time range.")]
  InvalidSaleTime,

  #[msg("Coin98Starship: Max amount reached")]
  MaxAmountReached,

  #[msg("Coin98Starship: Min amount not satisfied.")]
  MinAmountNotSatisfied,

  #[msg("Coin98Starship: Only allowed during registration time.")]
  NotInRegistrationTime,

  #[msg("Coin98Starship: Only allowed during sale time.")]
  NotInSaleTime,

  #[msg("Coin98Starship: Not registered.")]
  NotRegistered,

  #[msg("Coin98Starship: Not allowed.")]
  NotWhitelisted,

  #[msg("Coin98Starship: Registration and sale time overlap.")]
  RegistrationAndSaleTimeOverlap,

  #[msg("Coin98Starship: Redeem by SOL not allowed.")]
  RedeemBySolNotAllowed,

  #[msg("Coin98Starship: Redeem by token not allowed.")]
  RedeemByTokenNotAllowed,
}
