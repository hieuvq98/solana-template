use anchor_lang::prelude::*;

mod events;
mod instructions;
mod state;
mod utils;
mod constant;
mod error;

use constant::*;
use instructions::*;
use error::ErrorCode;

declare_id!("MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD");

#[program]
pub mod coin98_starship {

    use super::*;

    #[access_control(verify_root(*ctx.accounts.root.key))]
    pub fn init(ctx: Context<Init>, fee_owner: Pubkey) -> Result<()>{
        let app_data = &mut ctx.accounts.app_data;

        app_data.fee_owner = fee_owner;
        app_data.nonce = ctx.bumps.app_data;

        Ok(())
    }

    #[access_control(verify_root(*ctx.accounts.root.key))]
    pub fn set_fee_receiver(ctx: Context<SetFeeReceiver>, fee_owner: Pubkey) -> Result<()>{
        let app_data = &mut ctx.accounts.app_data;

        app_data.fee_owner = fee_owner;
        Ok(())
    }

    #[access_control(verify_root(*ctx.accounts.root.key))]
    pub fn create_sale_standard(
        ctx: Context<CreateSaleStandard>,
        launchpad_path: Vec<u8>,
        token_mint: Pubkey,
        owner: Pubkey,
        protocol_fee: u64
    ) -> Result<()> {
        let nonce = ctx.bumps.launchpad;
        ctx.accounts.process(launchpad_path, token_mint, owner, protocol_fee, nonce, ctx.program_id)
    }

    #[access_control(verify_root(*ctx.accounts.root.key))]
    pub fn set_sale_standard_protocol_fee(
        ctx: Context<SetSaleStandardProtocolFee>,
        protocol_fee: u64
    ) -> Result<()> {
        let launchpad = &mut ctx.accounts.launchpad;
        launchpad.set_protocol_fee(protocol_fee);
        Ok(())
    }

    #[access_control(verify_root(*ctx.accounts.root.key))]
    pub fn set_sale_sequenced_protocol_fee(
        ctx: Context<SetSaleSequencedProtocolFee>,
        protocol_fee: u64
    ) -> Result<()> {
        let launchpad = &mut ctx.accounts.launchpad;
        launchpad.set_protocol_fee(protocol_fee);
        Ok(())
    }

    // pub fn create_sale_sequenced() {}

    // pub fn create_pool() {}

    // pub fn set_admin() {}

    // pub fn set_fee_receiver() {}

    // pub fn set_white_list() {}

    // pub fn update_protocol_fee() {}
}

pub fn verify_root(user: Pubkey) -> Result<()> {
    let user_key = user.to_string();
    let result = ROOT_KEYS.iter().position(|&key| key == &user_key[..]);
    if result == None {
      return Err(ErrorCode::Unauthorized.into());
    }
  
    Ok(())
}