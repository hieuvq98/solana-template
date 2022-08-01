use anchor_lang::prelude::*;

#[account]
pub struct Launchpad {
  pub nonce: u8,
  pub price_in_sol_n: u64,
  pub price_in_sol_d: u64,
  pub price_in_token_n: u64,
  pub price_in_token_d: u64,
  pub token_program_id: Pubkey,
  pub token0_mint: Pubkey,
  pub token1_mint: Pubkey,
  pub vault_program_id: Pubkey,
  pub vault: Pubkey,
  pub vault_signer: Pubkey,
  pub vault_token0: Pubkey,
  pub vault_token1: Pubkey,
  pub is_private_sale: bool,
  pub private_sale_signature: Vec<u8>,
  pub min_per_tx: u64,
  pub max_per_user: u64,
  pub register_start_timestamp: i64,
  pub register_end_timestamp: i64,
  pub redeem_start_timestamp: i64,
  pub redeem_end_timestamp: i64,
  pub owner: Pubkey, // For compability reason
  pub new_owner: Pubkey, // For compability reason
  pub is_active: bool,
}
impl Launchpad {
  pub const LEN: usize = 16 + 1 + 8 + 8 + 8 + 8 + 32 + 32 + 32 + 32 + 32 + 32 + 32 + 32 + 1 + 36 + 8 + 8 + 8 + 8 + 8 + 8 + 32 + 32 + 1;
}

#[account]
pub struct GlobalProfile {
  pub nonce: u8,
  pub user: Pubkey,
  pub is_blacklisted: bool,
}
impl GlobalProfile {
  pub const LEN: usize = 16 + 1 + 32 + 1;
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
  pub const LEN: usize = 16 + 1 + 32 + 32 + 1 + 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Default)]
pub struct WhitelistParams {
  pub index: u32,
  pub address: Pubkey,
}
