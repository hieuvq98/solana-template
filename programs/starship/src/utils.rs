use anchor_lang::prelude::*;

use crate::constants::ROOT_KEYS;
use crate::ErrorCode;

pub fn verify_root(user: Pubkey) -> Result<()> {
  let user_key = user.to_string();
  let result = ROOT_KEYS.iter().position(|&key| key == &user_key[..]);
  if result == None {
    return Err(ErrorCode::InvalidOwner.into());
  }

  Ok(())
}
