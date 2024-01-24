use anchor_lang::prelude::*;

#[event]
pub struct CreateLaunchpadEvent {
  pub launchpad_path: Vec<u8>,
  pub token_mint: Pubkey
}