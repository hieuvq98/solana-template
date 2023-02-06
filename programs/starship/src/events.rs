use anchor_lang::prelude::*;

#[event]
pub struct CreateLaunchpadEvent {
  pub launchpad_path: Vec<u8>,
  pub token_mint: Pubkey
}

#[event]
pub struct SetLaunchpadEvent {
  pub price_n: u64,
  pub price_d: u64,
  pub min_per_tx: u64,
  pub max_per_user: u64,
  pub limit_sale: u64,
  pub register_start_timestamp: i64,
  pub register_end_timestamp: i64,
  pub redeem_start_timestamp: i64,
  pub redeem_end_timestamp: i64,
  pub private_sale_root: Option<[u8; 32]>,
}

#[event]
pub struct UpdateProtocolFeeEvent {
  pub protocol_fee: u64
}

#[event]
pub struct UpdateSharingFeeEvent {
  pub sharing_fee: u64
}

#[event]
pub struct TransferLaunchpadOwnershipEvent {
  pub new_owner: Pubkey
}

#[event]
pub struct AcceptLaunchpadOwnershipEvent {
  pub new_owner: Pubkey
}

#[event]
pub struct CreateLaunchpadPuchaseEvent {
  pub token_mint: Pubkey
}

#[event]
pub struct SetLaunchpadPuchaseEvent {
  pub price_n: u64,
  pub price_d: u64,
  pub min_per_tx: u64,
  pub max_per_user: u64,
  pub limit_sale: u64,
}

#[event]
pub struct SetLaunchpadStatusEvent {
  pub is_active: bool
}

#[event]
pub struct RegisterEvent {
  pub index: u32,
  pub proofs: Vec<[u8; 32]>
}

#[event]
pub struct RedeemBySolEvent {
  pub amount: u64
}

#[event]
pub struct RedeemByTokenEvent {
  pub amount: u64
}

#[event]
pub struct SetBlacklistEvent {
  pub user: Pubkey,
  pub is_blacklisted: bool
}

#[event]
pub struct CreateUserProfileEvent {
  pub user: Pubkey,
}

#[event]
pub struct WithdrawSolEvent {
  pub amount: u64,
}

#[event]
pub struct WithdrawTokenEvent {
  pub amount: u64,
}
