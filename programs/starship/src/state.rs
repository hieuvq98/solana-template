use anchor_lang::prelude::*;

#[account]
pub struct Launchpad {
  pub nonce: u8,
  pub signer_nonce: u8,
  pub is_active: bool,
  pub price_n: u64,
  pub price_d: u64,
  pub min_per_tx: u64,
  pub max_per_user: u64,
  pub total_limit: u64,
  pub total_sold: u64,
  pub total_claimed: u64,
  pub amount_sold_in_sol: u64,
  pub amount_limit_in_sol: u64,
  pub register_start_timestamp: i64,
  pub register_end_timestamp: i64,
  pub redeem_start_timestamp: i64,
  pub redeem_end_timestamp: i64,
  pub claim_start_timestamp: i64,
  pub whitelist_authority: Option<Pubkey>,
  pub token_mint: Pubkey,
  pub owner: Pubkey,
  pub new_owner: Pubkey,
  pub protocol_fee: u64,
  pub sharing_fee: u64,
}
impl Launchpad {
  pub const LEN: usize = 1 + 1 + 1 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 33 + 32 + 32 + 32 + 8 + 8;
}

#[account]
pub struct LaunchpadPurchase {
  pub nonce: u8,
  pub price_n: u64,
  pub price_d: u64,
  pub min_per_tx: u64,
  pub max_per_user: u64,
  pub amount_sold_in_token: u64,
  pub amount_limit_in_token: u64,
  pub launchpad: Pubkey,
  pub token_mint: Pubkey,
  pub sharing_fee: u64,
}

impl LaunchpadPurchase {
  pub const LEN: usize = 1 + 8 + 8 + 8 + 8 + 8 + 8 + 32 + 32 + 8;
}

#[account]
pub struct UserProfile {
  pub nonce: u8,
  pub launchpad: Pubkey,
  pub user: Pubkey,
  pub is_registered: bool,
  pub redeemed_token: u64,
  pub pending_token: u64,
}
impl UserProfile {
  pub const LEN: usize = 1 + 32 + 32 + 1 + 8 + 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Default)]
pub struct WhitelistParams {
  pub launchpad: Pubkey,
  pub address: Pubkey,
}

#[account]
pub struct WhitelistToken {
  pub nonce: u8
}

impl WhitelistToken {
  pub const LEN: usize = 8;
}
