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

    pub fn init(ctx: Context<Init>) -> Result<()>{
        Ok(())
    }

    pub fn create_sale_standard(ctx: Context<CreateSaleStandard>) -> Result<()> {
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