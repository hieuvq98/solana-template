use anchor_lang::prelude::*;

#[account]
pub struct ExchangeInfo {
    pub nonce: u8,
    pub price_n: u64,
    pub price_d: u64,
    pub max_amount: u64,
    pub sharing_fee: u64,
    pub launchpad: Pubkey,
    pub token_mint: Pubkey,
}

impl ExchangeInfo {
    pub const LEN: usize = 1 + 8 + 8 + 8 + 8 + 32 + 32;
}

#[account]
pub struct SaleStandard {
    //launchpad info
    pub nonce: u8,
    pub signer_nonce: u8,
    pub token_mint: Pubkey,
    pub register_start_timestamp: u64,
    pub register_end_timestamp: u64,
    pub redeem_start_timestamp: u64,
    pub redeem_end_timestamp: u64,
    pub claim_start_timestamp: u64,
    pub claim_end_timestamp: u64,
    pub whitelist_authority: Option<Pubkey>,
    pub owner: Pubkey,
    pub protocol_fee: u64,
    //launchpad status
    pub is_active: bool,
    pub total_sold: u64,
    pub total_claimed: u64,
    pub total_register: u64,
}

impl SaleStandard {
    pub const LEN: usize = 8 + 8 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 32 + 32 + 8 + 1 + 8 + 8 + 8;
}
    

