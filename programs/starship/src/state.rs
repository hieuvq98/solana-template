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
  pub limit_sale: u64,
  pub register_start_timestamp: i64,
  pub register_end_timestamp: i64,
  pub redeem_start_timestamp: i64,
  pub redeem_end_timestamp: i64,
  pub private_sale_root: Option<Vec<u8>>,
  pub token_mint: Pubkey,
  pub owner: Pubkey, // For compability reason
  pub new_owner: Pubkey, // For compability reason
  pub protocol_fee: u64,
  pub sharing_fee: u64,
}
impl Launchpad {
  pub const LEN: usize = 1 + 1 + 1 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 32 + 32 + 32 + 32 + 8 + 8;
}

#[account]
pub struct LaunchpadPurchase {
  pub nonce: u8,
  pub price_n: u64,
  pub price_d: u64,
  pub min_per_tx: u64,
  pub max_per_user: u64,
  pub limit_sale: u64,
  pub launchpad: Pubkey,
  pub token_mint: Pubkey,
}

impl LaunchpadPurchase {
  pub const LEN: usize = 1 + 8 + 8 + 8 + 8 + 8 + 32 + 32;
}

#[account]
pub struct GlobalProfile {
  pub nonce: u8,
  pub user: Pubkey,
  pub is_blacklisted: bool,
}
impl GlobalProfile {
  pub const LEN: usize = 1 + 32 + 1;
}

#[account]
pub struct LocalProfile {
  pub nonce: u8,
  pub launchpad: Pubkey,
  pub user: Pubkey,
  pub is_registered: bool,
  pub redeemed_token: u64,
}
impl LocalProfile {
  pub const LEN: usize = 1 + 32 + 32 + 1 + 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Default)]
pub struct WhitelistParams {
  pub index: u32,
  pub address: Pubkey,
}
