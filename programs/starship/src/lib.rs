pub mod constant;
pub mod context;
pub mod error;
pub mod shared;
pub mod state;
pub mod external;

use anchor_lang::prelude::*;
use solana_program::keccak::hash;
use std::convert::TryInto;
use crate::error::ErrorCode;
use crate::constant::{
  ROOT_KEYS,
  SIGNER_SEED_1,
};
use crate::context::*;
use crate::state::WhitelistParams;
use crate::external::anchor_spl_system::transfer_lamport;
use crate::external::anchor_spl_token::transfer_token;
use crate::external::coin98_vault::withdraw_token;

declare_id!("SS4VMP9wmqQdehu7Uc6g1Ymsx4BCVVghKp4wRmmy1jj");

#[program]
mod coin98_starship {
  use super::*;

  #[access_control(verify_root(*ctx.accounts.root.key))]
  pub fn create_launchpad(
    ctx: Context<CreateLaunchpadContext>,
    _launchpad_path: Vec<u8>,
  ) -> Result<()> {

    let launchpad = &mut ctx.accounts.launchpad;

    launchpad.nonce = *ctx.bumps.get("launchpad").unwrap();
    let (_, signer_nonce) = Pubkey::find_program_address(
      &[
        &SIGNER_SEED_1,
        &launchpad.key().to_bytes(),
      ],
      ctx.program_id,
    );
    launchpad.signer_nonce = signer_nonce;
    launchpad.owner = *ctx.accounts.root.key;

    Ok(())
  }

  #[access_control(verify_root(*ctx.accounts.root.key))]
  pub fn set_launchpad(
    ctx: Context<SetLaunchpadContext>,
    price_in_sol_n: u64,
    price_in_sol_d: u64,
    price_in_token_n: u64,
    price_in_token_d: u64,
    token0_mint: Pubkey,
    token1_mint: Pubkey,
    vault_program_id: Pubkey,
    vault: Pubkey,
    vault_signer: Pubkey,
    vault_token0: Pubkey,
    vault_token1: Pubkey,
    is_private_sale: bool,
    private_sale_signature: [u8; 32],
    min_per_tx: u64,
    max_per_user: u64,
    register_start_timestamp: i64,
    register_end_timestamp: i64,
    redeem_start_timestamp: i64,
    redeem_end_timestamp: i64,
  ) -> Result<()> {

    let clock = Clock::get().unwrap();

    require!(register_start_timestamp < register_end_timestamp, ErrorCode::InvalidInput);
    require!(clock.unix_timestamp < register_start_timestamp, ErrorCode::InvalidInput);
    require!(redeem_start_timestamp < redeem_end_timestamp, ErrorCode::InvalidInput);
    require!(register_end_timestamp < redeem_start_timestamp, ErrorCode::TimeOverlap);

    let launchpad = &mut ctx.accounts.launchpad;

    launchpad.price_in_sol_n = price_in_sol_n;
    launchpad.price_in_sol_d = price_in_sol_d;
    launchpad.price_in_token_n = price_in_token_n;
    launchpad.price_in_token_d = price_in_token_d;
    launchpad.token0_mint = token0_mint;
    launchpad.token1_mint = token1_mint;
    launchpad.vault_program_id = vault_program_id;
    launchpad.vault = vault;
    launchpad.vault_signer = vault_signer;
    launchpad.vault_token0 = vault_token0;
    launchpad.vault_token1 = vault_token1;
    launchpad.is_private_sale = is_private_sale;
    launchpad.private_sale_signature = private_sale_signature.try_to_vec().unwrap();
    launchpad.min_per_tx = min_per_tx;
    launchpad.max_per_user = max_per_user;
    launchpad.register_start_timestamp = register_start_timestamp;
    launchpad.register_end_timestamp = register_end_timestamp;
    launchpad.redeem_start_timestamp = redeem_start_timestamp;
    launchpad.redeem_end_timestamp = redeem_end_timestamp;

    Ok(())
  }

  #[access_control(verify_root(*ctx.accounts.root.key))]
  pub fn set_launchpad_status(
    ctx: Context<SetLaunchpadStatusContext>,
    is_active: bool,
  ) -> Result<()> {

    let launchpad = &mut ctx.accounts.launchpad;

    launchpad.is_active = is_active;

    Ok(())
  }

  pub fn register(
    ctx: Context<RegisterContext>,
    index: u32,
    proofs: Vec<[u8; 32]>,
  ) -> Result<()> {

    let user = &ctx.accounts.user;
    let launchpad = &ctx.accounts.launchpad;
    let global_profile = &ctx.accounts.global_profile;
    let clock = Clock::get().unwrap();

    require!(!global_profile.is_blacklisted, ErrorCode::Forbidden);
    require!(
      clock.unix_timestamp >= launchpad.register_start_timestamp && clock.unix_timestamp <= launchpad.register_end_timestamp,
      ErrorCode::NotInTimeframe,
    );
    if launchpad.is_private_sale {
      let whitelist = WhitelistParams {
        index: index,
        address: *user.key
      };
      let whitelist_data = whitelist.try_to_vec().unwrap();
      let leaf = hash(&whitelist_data[..]);
      let root: [u8; 32] = launchpad.private_sale_signature.clone().try_into().unwrap();
      let is_valid_proof = shared::verify_proof(proofs, root, leaf.to_bytes());
      require!(is_valid_proof, ErrorCode::Unauthorized);
    }

    let local_profile = &mut ctx.accounts.local_profile;
    local_profile.is_registered = true;

    Ok(())
  }

  pub fn redeem_by_sol(
    ctx: Context<RedeemBySolContext>,
    amount: u64,
  ) -> Result<()> {

    let user = &ctx.accounts.user;
    let launchpad = &ctx.accounts.launchpad;
    let launchpad_signer = &ctx.accounts.launchpad_signer;
    let global_profile = &ctx.accounts.global_profile;
    let local_profile = &ctx.accounts.local_profile;
    let user_token1 = &ctx.accounts.user_token1;
    let vault = &ctx.accounts.vault;
    let vault_signer = &ctx.accounts.vault_signer;
    let vault_token1 = &ctx.accounts.vault_token1;
    let clock = Clock::get().unwrap();
    let vault_program = &ctx.accounts.vault_program;
    let token_program = &ctx.accounts.token_program;

    require!(!global_profile.is_blacklisted, ErrorCode::Forbidden);
    require!(launchpad.price_in_sol_n > 0u64, ErrorCode::NotAllowed);
    require!(local_profile.is_registered, ErrorCode::Unauthorized);
    require!(
      clock.unix_timestamp >= launchpad.redeem_start_timestamp && clock.unix_timestamp <= launchpad.redeem_end_timestamp,
      ErrorCode::NotInTimeframe,
    );
    require!(
      launchpad.min_per_tx == 0u64 || amount >= launchpad.min_per_tx,
      ErrorCode::MinAmountNotSatisfied,
    );
    let redeemed_amount = local_profile.redeemed_token.checked_add(amount).unwrap();
    require!(
      launchpad.max_per_user == 0u64 || redeemed_amount <= launchpad.max_per_user,
      ErrorCode::MaxAmountReached,
    );

    let amount_sol = shared::calculate_sub_total(amount, launchpad.price_in_sol_n, launchpad.price_in_sol_d)
      .unwrap();
    // Transfer lamports
    transfer_lamport(
        &user.to_account_info(),
        &vault_signer.to_account_info(),
        amount_sol,
        &[]
      )
      .expect("Starship: CPI failed.");

    let local_profile = &mut ctx.accounts.local_profile;

    local_profile.redeemed_token = redeemed_amount;
    let seeds: &[&[_]] = &[
      &SIGNER_SEED_1,
      launchpad.to_account_info().key.as_ref(),
      &[launchpad.signer_nonce],
    ];

    // Transfer token 1
    withdraw_token(
        &amount,
        &launchpad_signer.to_account_info(),
        &vault.to_account_info(),
        &vault_signer.to_account_info(),
        &vault_token1.to_account_info(),
        &user_token1.to_account_info(),
        &vault_program.to_account_info(),
        &token_program.to_account_info(),
        &[seeds],
      )
      .expect("Starship: CPI failed.");

    Ok(())
  }

  pub fn redeem_by_token(
    ctx: Context<RedeemByTokenContext>,
    amount: u64,
  ) -> Result<()> {

    let user = &ctx.accounts.user;
    let launchpad = &ctx.accounts.launchpad;
    let launchpad_signer = &ctx.accounts.launchpad_signer;
    let global_profile = &ctx.accounts.global_profile;
    let local_profile = &ctx.accounts.local_profile;
    let user_token0 = &ctx.accounts.user_token0;
    let user_token1 = &ctx.accounts.user_token1;
    let vault = &ctx.accounts.vault;
    let vault_signer = &ctx.accounts.vault_signer;
    let vault_token0 = &ctx.accounts.vault_token0;
    let vault_token1 = &ctx.accounts.vault_token1;
    let clock = Clock::get().unwrap();
    let vault_program = &ctx.accounts.vault_program;
    let token_program = &ctx.accounts.token_program;

    require!(!global_profile.is_blacklisted, ErrorCode::Forbidden);
    require!(launchpad.price_in_token_n > 0u64, ErrorCode::NotAllowed);
    require!(local_profile.is_registered, ErrorCode::Unauthorized);
    require!(
      clock.unix_timestamp >= launchpad.redeem_start_timestamp && clock.unix_timestamp <= launchpad.redeem_end_timestamp,
      ErrorCode::NotInTimeframe,
    );
    require!(
      launchpad.min_per_tx == 0u64 || amount >= launchpad.min_per_tx,
      ErrorCode::MinAmountNotSatisfied,
    );
    let redeemed_amount = local_profile.redeemed_token.checked_add(amount).unwrap();
    require!(
      launchpad.max_per_user == 0u64 || redeemed_amount <= launchpad.max_per_user,
      ErrorCode::MaxAmountReached,
    );

    let amount_token0 = shared::calculate_sub_total(amount, launchpad.price_in_token_n, launchpad.price_in_token_d)
      .unwrap();

    // Transfer token 0
    transfer_token(
        &user.to_account_info(),
        &user_token0.to_account_info(),
        &vault_token0.to_account_info(),
        amount_token0,
        &[]
      )
      .expect("Starship: CPI failed.");

    let local_profile = &mut ctx.accounts.local_profile;

    local_profile.redeemed_token = redeemed_amount;

    let seeds: &[&[_]] = &[
      &SIGNER_SEED_1,
      launchpad.to_account_info().key.as_ref(),
      &[launchpad.signer_nonce],
    ];

    // Transfer token 1
    withdraw_token(
        &amount,
        &launchpad_signer.to_account_info(),
        &vault.to_account_info(),
        &vault_signer.to_account_info(),
        &vault_token1.to_account_info(),
        &user_token1.to_account_info(),
        &vault_program.to_account_info(),
        &token_program.to_account_info(),
        &[seeds],
      )
      .expect("Starship: CPI failed.");

    Ok(())
  }

  #[access_control(verify_root(*ctx.accounts.root.key))]
  pub fn set_blacklist(
    ctx: Context<SetBlacklistContext>,
    _user: Pubkey,
    is_blacklisted: bool,
  ) -> Result<()> {

    let profile = &mut ctx.accounts.global_profile;

    profile.is_blacklisted = is_blacklisted;

    Ok(())
  }

  pub fn create_global_profile(
    ctx: Context<CreateGlobalProfileContext>,
    user: Pubkey,
  ) -> Result<()> {

    let profile = &mut ctx.accounts.global_profile;

    profile.nonce = *ctx.bumps.get("global_profile").unwrap();
    profile.user = user;

    Ok(())
  }

  pub fn create_local_profile(
    ctx: Context<CreateLocalProfileContext>,
    user: Pubkey,
  ) -> Result<()> {

    let launchpad = &ctx.accounts.launchpad;

    let profile = &mut ctx.accounts.local_profile;

    profile.nonce = *ctx.bumps.get("local_profile").unwrap();
    profile.launchpad = launchpad.key();
    profile.user = user;

    Ok(())
  }
}

pub fn verify_root(user: Pubkey) -> Result<()> {
  let user_key = user.to_string();
  let result = ROOT_KEYS.iter().position(|&key| key == &user_key[..]);
  if result == None {
    return Err(ErrorCode::Unauthorized.into());
  }

  Ok(())
}
