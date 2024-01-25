use anchor_lang::prelude::*;


#[event]
pub struct CreateLaunchpadEvent {
  pub launchpad_path: Vec<u8>,
  pub token_mint: Pubkey
}

#[event]
pub struct CreateExchangeInfoEvent {
  pub launchpad: Vec<u8>,
  pub token_mint: Pubkey,
  pub price_n: u64,
  pub price_d: u64,
  pub max_amount: u64,
  pub sharing_fee: u64,
}