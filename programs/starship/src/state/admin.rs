use anchor_lang::prelude::*;

#[account]
pub struct AppData {
    pub nonce: u8,
    pub fee_owner: Pubkey,
}

impl AppData {
    pub const LEN: usize = 1 + 32;
}