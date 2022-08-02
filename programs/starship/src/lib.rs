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

declare_id!("FaJtq6SLQNwGgaggr7izJMgRYkxU1xwtCjnyESSXhvHG");

#[program]
mod coin98_starship {
  use super::*;

  #[access_control(verify_root(*ctx.accounts.root.key))]
  pub fn create_launchpad(
    ctx: Context<CreateLaunchpadContext>,
    _launchpad_path: Vec<u8>,
    token_mint: Pubkey,
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
    launchpad.token_mint = token_mint;

    Ok(())
  }

  #[access_control(verify_root(*ctx.accounts.root.key))]
  pub fn set_launchpad(
    ctx: Context<SetLaunchpadContext>,
    price_n: u64,
    price_d: u64,
    min_per_tx: u64,
    max_per_user: u64,
    limit_sale: u64,
    register_start_timestamp: i64,
    register_end_timestamp: i64,
    redeem_start_timestamp: i64,
    redeem_end_timestamp: i64,
    private_sale_root: Option<[u8; 32]>,
  ) -> Result<()> {

    let clock = Clock::get().unwrap();

    /*
    require!(register_start_timestamp < register_end_timestamp, ErrorCode::InvalidInput);
    require!(clock.unix_timestamp < register_start_timestamp, ErrorCode::InvalidInput);
    require!(redeem_start_timestamp < redeem_end_timestamp, ErrorCode::InvalidInput);
    require!(register_end_timestamp < redeem_start_timestamp, ErrorCode::TimeOverlap);
    */

    let launchpad = &mut ctx.accounts.launchpad;

    launchpad.price_n = price_n;
    launchpad.price_d = price_d;
    launchpad.min_per_tx = min_per_tx;
    launchpad.max_per_user = max_per_user;
    launchpad.limit_sale = limit_sale;
    launchpad.register_start_timestamp = register_start_timestamp;
    launchpad.register_end_timestamp = register_end_timestamp;
    launchpad.redeem_start_timestamp = redeem_start_timestamp;
    launchpad.redeem_end_timestamp = redeem_end_timestamp;

    if let Some(root) = private_sale_root {
      launchpad.private_sale_root = Some(root.to_vec());
    } else {
      launchpad.private_sale_root = None;
    }

    Ok(())
  }

  #[access_control(verify_root(*ctx.accounts.root.key))]
  pub fn create_launchpad_purchase(
    ctx: Context<CreateLaunchpadPurchaseContext>,
    token_mint: Pubkey,
  ) -> Result<()> {
    let launchpad = &ctx.accounts.launchpad;
    let launchpad_purchase = &mut ctx.accounts.launchpad_purchase;

    launchpad_purchase.nonce = *ctx.bumps.get("launchpad_purchase").unwrap();
    launchpad_purchase.token_mint = token_mint;
    launchpad_purchase.launchpad = launchpad.key();

    Ok(())
  }

  #[access_control(verify_root(*ctx.accounts.root.key))]
  pub fn set_launchpad_purchase(
    ctx: Context<SetLaunchPadPurchaseContext>,
    limit_sale: u64,
    price_n: u64,
    price_d: u64,
    min_per_tx: u64,
    max_per_user: u64
  ) -> Result<()> {
    let launchpad_purchase = &mut ctx.accounts.launchpad_purchase;

    launchpad_purchase.limit_sale = limit_sale;
    launchpad_purchase.price_n = price_n;
    launchpad_purchase.price_d = price_d;
    launchpad_purchase.min_per_tx = min_per_tx;
    launchpad_purchase.max_per_user = max_per_user;

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
    /*
    require!(
      clock.unix_timestamp >= launchpad.register_start_timestamp && clock.unix_timestamp <= launchpad.register_end_timestamp,
      ErrorCode::NotInTimeframe,
    );
    */
    if let Some(root) = &launchpad.private_sale_root {
      let whitelist = WhitelistParams {
        index,
        address: *user.key
      };
      let whitelist_data = whitelist.try_to_vec().unwrap();
      let leaf = hash(&whitelist_data[..]);
      let is_valid_proof = shared::verify_proof(proofs, root.clone().try_into().unwrap(), leaf.to_bytes());
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
    let local_profile = &mut ctx.accounts.local_profile;
    let user_token_account = &ctx.accounts.user_token_account;
    let launchpad_token_account = &ctx.accounts.launchpad_token_account;
    let clock = Clock::get().unwrap();

    require!(!global_profile.is_blacklisted, ErrorCode::Forbidden);
    require!(launchpad.price_n > 0u64, ErrorCode::NotAllowed);
    require!(local_profile.is_registered, ErrorCode::Unauthorized);
    /*
    require!(
      clock.unix_timestamp >= launchpad.redeem_start_timestamp && clock.unix_timestamp <= launchpad.redeem_end_timestamp,
      ErrorCode::NotInTimeframe,
    );
    */
    require!(
      launchpad.min_per_tx == 0u64 || amount >= launchpad.min_per_tx,
      ErrorCode::MinAmountNotSatisfied,
    );
    let redeemed_amount = local_profile.redeemed_token.checked_add(amount).unwrap();
    require!(
      launchpad.max_per_user == 0u64 || redeemed_amount <= launchpad.max_per_user,
      ErrorCode::MaxAmountReached,
    );
    local_profile.redeemed_token = redeemed_amount;

    let amount_sol = shared::calculate_sub_total(amount, launchpad.price_n, launchpad.price_d)
      .unwrap();
    // Transfer lamports
    transfer_lamport(&user.to_account_info(), &launchpad_signer.to_account_info(), amount_sol, &[])
      .expect("Starship: CPI failed.");


    let seeds: &[&[_]] = &[
      &SIGNER_SEED_1,
      launchpad.to_account_info().key.as_ref(),
      &[launchpad.signer_nonce],
    ];

    // Transfer token 1
    transfer_token(launchpad_signer, &launchpad_token_account.to_account_info(), &user_token_account, amount, &[seeds])
      .expect("Starship: CPI failed.");

    Ok(())
  }

  pub fn redeem_by_token(
    ctx: Context<RedeemByTokenContext>,
    amount: u64,
  ) -> Result<()> {

    let user = &ctx.accounts.user;
    let launchpad = &ctx.accounts.launchpad;
    let launchpad_purchase = &mut ctx.accounts.launchpad_purchase;
    let launchpad_signer = &ctx.accounts.launchpad_signer;
    let global_profile = &ctx.accounts.global_profile;
    let local_profile = &ctx.accounts.local_profile;
    let user_token0_account = &ctx.accounts.user_token0_account;
    let user_token1_account = &ctx.accounts.user_token1_account;
    let launchpad_token0_account = &ctx.accounts.launchpad_token0_account;
    let launchpad_token1_account = &ctx.accounts.launchpad_token1_account;
    let clock = Clock::get().unwrap();

    require!(!global_profile.is_blacklisted, ErrorCode::Forbidden);
    require!(launchpad.price_n > 0u64, ErrorCode::NotAllowed);
    require!(local_profile.is_registered, ErrorCode::Unauthorized);
    /*
    require!(
      clock.unix_timestamp >= launchpad.redeem_start_timestamp && clock.unix_timestamp <= launchpad.redeem_end_timestamp,
      ErrorCode::NotInTimeframe,
    );
    */
    require!(
      launchpad.min_per_tx == 0u64 || amount >= launchpad.min_per_tx,
      ErrorCode::MinAmountNotSatisfied,
    );
    let redeemed_amount = local_profile.redeemed_token.checked_add(amount).unwrap();
    require!(
      launchpad.max_per_user == 0u64 || redeemed_amount <= launchpad.max_per_user,
      ErrorCode::MaxAmountReached,
    );

    let amount_token0 = shared::calculate_sub_total(amount, launchpad_purchase.price_n, launchpad_purchase.price_d)
      .unwrap();

    // Transfer token 0
    transfer_token(
        &user.to_account_info(),
        &user_token0_account.to_account_info(),
        &launchpad_token0_account.to_account_info(),
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
    transfer_token(
        &launchpad_signer,
        &launchpad_token1_account.to_account_info(),
        &user_token1_account.to_account_info(),
        amount,
        &[seeds]
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

  #[access_control(verify_root(*ctx.accounts.root.key))]
  pub fn withdraw_sol(
    ctx: Context<WithdrawSolContext>,
    amount: u64
  ) -> Result<()> {
    let root = &ctx.accounts.root;
    let launchpad = &ctx.accounts.launchpad;
    let launchpad_signer = &ctx.accounts.launchpad_signer;

    let seeds: &[&[_]] = &[
      &SIGNER_SEED_1,
      launchpad.to_account_info().key.as_ref(),
      &[launchpad.signer_nonce],
    ];

    transfer_lamport(&launchpad_signer.to_account_info(), &root.to_account_info(), amount, &[seeds])
      .expect("Starship: CPI failed.");

    Ok(())
  }

  #[access_control(verify_root(*ctx.accounts.root.key))]
  pub fn withdraw_token(
    ctx: Context<WithdrawTokenContext>,
    amount: u64
  ) -> Result<()> {
    let launchpad = &ctx.accounts.launchpad;
    let launchpad_signer = &ctx.accounts.launchpad_signer;
    let from = &ctx.accounts.from;
    let to = &ctx.accounts.to;

    let seeds: &[&[_]] = &[
      &SIGNER_SEED_1,
      launchpad.to_account_info().key.as_ref(),
      &[launchpad.signer_nonce],
    ];

    transfer_token(launchpad_signer, &from, &to, amount, &[seeds])
      .expect("Starship: CPI failed.");

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
