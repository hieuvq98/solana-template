use anchor_lang::prelude::*;

use crate::{
    error::{
        ErrorCode
    }
};

#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize)]
pub struct ExchangeInfo {
    pub nonce: u8,
    pub price_n: u64,
    pub price_d: u64,
    pub max_amount: u64,
    pub sharing_fee: u64,
    pub launchpad: Pubkey,
    pub token_mint: Pubkey,
}

#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize)]
pub struct LaunchpadInfo {
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
}

#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize)]
pub struct LaunchpadStatus {
    pub is_active: bool,
    pub total_sold: u64,
    pub total_claimed: u64,
    pub total_register: u64,
}

#[account]
pub struct SaleStandard {
    pub info: LaunchpadInfo,
    pub status: LaunchpadStatus,
    pub exchange_info: ExchangeInfo
}

impl SaleStandard {
    pub const LEN: usize = 8 + 8 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 32 + 32 + 8 + 1 + 8 + 8 + 8 + 1 + 8 + 8 + 8 + 8 + 32 + 32;

    pub fn set_protocol_fee(&mut self, protocol_fee: u64) -> Result<()> {
        require!(protocol_fee <= 2000, ErrorCode::MaxFeeReached);
        self.info.protocol_fee = protocol_fee;
        Ok(())
    }
}

#[account]
pub struct SaleSequenced {
    pub info: LaunchpadInfo,
    pub status: LaunchpadStatus,
    pub exchange_info: ExchangeInfo
}

impl SaleSequenced {
    pub const LEN: usize = 8 + 8 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 32 + 32 + 8 + 1 + 8 + 8 + 8 + 1 + 8 + 8 + 8 + 8 + 32 + 32;

    pub fn set_protocol_fee(&mut self, protocol_fee: u64) -> Result<()> {
        require!(protocol_fee <= 2000, ErrorCode::MaxFeeReached);
        self.info.protocol_fee = protocol_fee;
        Ok(())
    }
}

