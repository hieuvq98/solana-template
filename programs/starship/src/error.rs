use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {

  #[msg("Starship: Invalid account.")]
  InvalidAccount,

  #[msg("Starship: Invalid Fee.")]
  InvalidFee,

  #[msg("Starship: Invalid input.")]
  InvalidInput,

  #[msg("Starship: Launchpad inactive.")]
  LaunchpadInactive,

  #[msg("Starship: Launchpad started.")]
  LaunchpadStarted,

  #[msg("Starship: Max amount reached")]
  MaxAmountReached,

  #[msg("Starship: Min amount not satisfied.")]
  MinAmountNotSatisfied,

  #[msg("Starship: Not allowed.")]
  NotAllowed,

  #[msg("Starship: Not in timeframe.")]
  NotInTimeframe,

  #[msg("Starship: Timeframe is overlapped.")]
  TimeOverlap,

  #[msg("Starship: Not an owner.")]
  Unauthorized,

  #[msg("Starship: Max fee reached.")]
  MaxFeeReached,

  #[msg("Starship: Reach Limit Sold.")]
  ReachLimitSold,
}
