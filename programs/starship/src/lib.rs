pub mod constant;
pub mod context;
pub mod error;
pub mod external;
pub mod event;
pub mod state;
pub mod util;

use anchor_lang::prelude::*;
use solana_program::{
  keccak::{
    hash,
  },
  ed25519_program::{
    ID as ED25519_ID,
  },
  instruction::{
    Instruction,
  },
  sysvar::{
    instructions::{
      load_instruction_at_checked,
    },
  },
};
use crate::{
  constant::{
    ROOT_KEYS,
    SIGNER_SEED_1,
  },
  context::*,
  error::{
    ErrorCode,
  },
  event::*,
  state::{
    WhitelistParams,
  },
  util::{
    check_ed25519_data,
    calculate_out_total,
    calculate_sub_total,
    calculate_system_fee,
  }
};
use crate::external::{
  anchor_spl_system::{
    transfer_lamport,
  },
  anchor_spl_token::{
    transfer_token,
  },
};

#[cfg(feature = "mainnet")]
declare_id!("SPadMQQeXyUGprwgt2SM8xZWVdKJ82UpWstWBLEDJwj");

#[cfg(all(not(feature = "mainnet"), not(feature = "devnet")))]
declare_id!("SPDxTSk2fjE4jbZ9KuBkjZQHzEV2fkAHXh6sJ7oWbaj");

#[program]
mod coin98_starship {
  use super::*;

  #[access_control(verify_root(*ctx.accounts.root.key))]
  pub fn create_launchpad(
    ctx: Context<CreateLaunchpadContext>,
    launchpad_path: Vec<u8>,
    token_mint: Pubkey,
    owner: Pubkey,
    protocol_fee: u64,
    sharing_fee: u64
  ) -> Result<()> {
    require!(protocol_fee <= 2000, ErrorCode::MaxFeeReached);

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
    launchpad.owner = owner;
    launchpad.token_mint = token_mint;
    launchpad.protocol_fee = protocol_fee;
    launchpad.sharing_fee = sharing_fee;

    emit!(CreateLaunchpadEvent{
      launchpad_path,
      token_mint,
    });

    Ok(())
  }

  #[access_control(verify_root(*ctx.accounts.root.key))]
  pub fn create_whitelist_token(
    ctx: Context<CreateWhitelistTokenContext>,
    token_mint: Pubkey
  ) -> Result<()> {

    let whitelist = &mut ctx.accounts.whitelist;
    whitelist.nonce = *ctx.bumps.get("whitelist").unwrap();

    emit!(CreateWhitelistTokenEvent {
      token_mint,
    });

    Ok(())
  }

  #[access_control(verify_root(*ctx.accounts.root.key))]
  pub fn delete_whitelist_token(
    ctx: Context<DeleteWhitelistTokenContext>,
    token_mint: Pubkey
  ) -> Result<()> {

    emit!(DeleteWhitelistTokenEvent {
      token_mint
    });

    Ok(())
  }

  #[access_control(verify_owner(ctx.accounts.launchpad.owner, *ctx.accounts.owner.key))]
  pub fn set_launchpad(
    ctx: Context<SetLaunchpadContext>,
    price_n: u64,
    price_d: u64,
    min_per_tx: u64,
    max_per_user: u64,
    total_limit: u64,
    amount_limit_in_sol: u64,
    register_start_timestamp: i64,
    register_end_timestamp: i64,
    redeem_start_timestamp: i64,
    redeem_end_timestamp: i64,
    claim_start_timestamp: i64,
    whitelist_authority: Option<Pubkey>,
  ) -> Result<()> {

    let clock = Clock::get().unwrap();

    require!(price_d > 0u64 || price_n == 0u64, ErrorCode::InvalidInput);
    require!(register_start_timestamp < register_end_timestamp, ErrorCode::InvalidInput);
    require!(clock.unix_timestamp < register_start_timestamp, ErrorCode::InvalidInput);
    require!(redeem_start_timestamp < redeem_end_timestamp, ErrorCode::InvalidInput);
    require!(register_end_timestamp < redeem_start_timestamp, ErrorCode::TimeOverlap);

    let launchpad = &mut ctx.accounts.launchpad;

    require!(launchpad.register_start_timestamp == 0 || clock.unix_timestamp < launchpad.register_start_timestamp, ErrorCode::LaunchpadStarted);

    launchpad.is_active = true;
    launchpad.price_n = price_n;
    launchpad.price_d = price_d;
    launchpad.min_per_tx = min_per_tx;
    launchpad.max_per_user = max_per_user;
    launchpad.total_limit = total_limit;
    launchpad.amount_limit_in_sol = amount_limit_in_sol;
    launchpad.register_start_timestamp = register_start_timestamp;
    launchpad.register_end_timestamp = register_end_timestamp;
    launchpad.redeem_start_timestamp = redeem_start_timestamp;
    launchpad.redeem_end_timestamp = redeem_end_timestamp;
    launchpad.claim_start_timestamp = claim_start_timestamp;
    launchpad.whitelist_authority = whitelist_authority;

    emit!(SetLaunchpadEvent{
      price_n,
      price_d,
      min_per_tx,
      max_per_user,
      total_limit,
      amount_limit_in_sol,
      register_start_timestamp,
      register_end_timestamp,
      redeem_start_timestamp,
      redeem_end_timestamp,
      whitelist_authority,
    });

    Ok(())
  }

  #[access_control(verify_root(*ctx.accounts.root.key))]
  pub fn update_protocol_fee(
    ctx: Context<UpdateProtocolFeeContext>,
    protocol_fee: u64
  ) -> Result<()> {
    require!(protocol_fee <= 2000, ErrorCode::MaxFeeReached);
    let launchpad = &mut ctx.accounts.launchpad;

    launchpad.protocol_fee = protocol_fee;

    emit!(UpdateProtocolFeeEvent{
      protocol_fee
    });

    Ok(())
  }

  #[access_control(verify_owner(ctx.accounts.launchpad.owner, *ctx.accounts.owner.key))]
  pub fn update_sharing_fee(
    ctx: Context<UpdateSharingFeeContext>,
    sharing_fee: u64
  ) -> Result<()> {
    let launchpad = &mut ctx.accounts.launchpad;

    launchpad.sharing_fee = sharing_fee;

    emit!(UpdateSharingFeeEvent{
      sharing_fee
    });

    Ok(())
  }

  #[access_control(verify_owner(ctx.accounts.launchpad.owner, *ctx.accounts.owner.key))]
  pub fn transfer_launchpad_ownership(
    ctx: Context<TransferLaunchpadOwnershipContext>,
    new_owner: Pubkey
  ) -> Result<()> {
    let launchpad = &mut ctx.accounts.launchpad;

    launchpad.new_owner = new_owner;

    emit!(TransferLaunchpadOwnershipEvent{
      new_owner
    });

    Ok(())
  }

  #[access_control(verify_owner(ctx.accounts.launchpad.new_owner, *ctx.accounts.new_owner.key))]
  pub fn accept_launchpad_ownership(
    ctx: Context<AcceptLaunchpadOwnershipContext>,
  ) -> Result<()> {
    let launchpad = &mut ctx.accounts.launchpad;
    let new_owner = &ctx.accounts.new_owner;

    launchpad.owner = new_owner.key();
    launchpad.new_owner = Pubkey::default();

    emit!(AcceptLaunchpadOwnershipEvent{
      new_owner: new_owner.key()
    });

    Ok(())
  }

  #[access_control(verify_owner(ctx.accounts.launchpad.owner, *ctx.accounts.owner.key))]
  pub fn create_launchpad_purchase(
    ctx: Context<CreateLaunchpadPurchaseContext>,
    token_mint: Pubkey,
  ) -> Result<()> {
    let launchpad = &ctx.accounts.launchpad;
    let launchpad_purchase = &mut ctx.accounts.launchpad_purchase;

    launchpad_purchase.nonce = *ctx.bumps.get("launchpad_purchase").unwrap();
    launchpad_purchase.launchpad = launchpad.key();
    launchpad_purchase.token_mint = token_mint;

    emit!(CreateLaunchpadPuchaseEvent{
      token_mint,
    });

    Ok(())
  }

  #[access_control(verify_owner(ctx.accounts.launchpad.owner, *ctx.accounts.owner.key))]
  pub fn set_launchpad_purchase(
    ctx: Context<SetLaunchPadPurchaseContext>,
    price_n: u64,
    price_d: u64,
    min_per_tx: u64,
    max_per_user: u64,
    amount_limit_in_token: u64,
    sharing_fee: u64
  ) -> Result<()> {

    require!(price_d > 0u64 || price_n == 0u64, ErrorCode::InvalidInput);

    let launchpad_purchase = &mut ctx.accounts.launchpad_purchase;

    launchpad_purchase.price_n = price_n;
    launchpad_purchase.price_d = price_d;
    launchpad_purchase.min_per_tx = min_per_tx;
    launchpad_purchase.max_per_user = max_per_user;
    launchpad_purchase.amount_limit_in_token = amount_limit_in_token;
    launchpad_purchase.sharing_fee = sharing_fee;

    emit!(SetLaunchpadPuchaseEvent{
      price_n,
      price_d,
      min_per_tx,
      max_per_user,
      amount_limit_in_token,
    });

    Ok(())
  }

  #[access_control(verify_owner(ctx.accounts.launchpad.owner, *ctx.accounts.owner.key))]
  pub fn set_launchpad_status(
    ctx: Context<SetLaunchpadStatusContext>,
    is_active: bool,
  ) -> Result<()> {

    let launchpad = &mut ctx.accounts.launchpad;

    launchpad.is_active = is_active;

    emit!(SetLaunchpadStatusEvent{
      is_active,
    });
    Ok(())
  }

  pub fn register(
    ctx: Context<RegisterContext>,
    whitelist_signature: [u8; 64]
  ) -> Result<()> {

    let user = &ctx.accounts.user;
    let launchpad = &ctx.accounts.launchpad;
    let clock = Clock::get().unwrap();

    require!(launchpad.is_active, ErrorCode::LaunchpadInactive);
    require!(
      clock.unix_timestamp >= launchpad.register_start_timestamp && clock.unix_timestamp <= launchpad.register_end_timestamp,
      ErrorCode::NotInTimeframe,
    );
    if let Some(authority) = &launchpad.whitelist_authority {
      let ix: Instruction = load_instruction_at_checked(0, &ctx.accounts.sysvar_program)?;

      require!(ix.program_id == ED25519_ID, ErrorCode::InvalidValidateSignInstruction);
      require!(ix.accounts.len() == 0, ErrorCode::InvalidValidateSignInstruction);

      let whitelist = WhitelistParams {
        launchpad: launchpad.key(),
        address: user.key()
      }.try_to_vec().unwrap();
      let hashed_message = hash(&whitelist[..]).to_bytes();

      require!(ix.data.len() == (16 + 64 + 32 + hashed_message.len()), ErrorCode::InvalidValidateSignInstruction);
      check_ed25519_data(&ix.data, authority.as_ref(), &hashed_message, &whitelist_signature)?;
    }

    let user_profile = &mut ctx.accounts.user_profile;
    user_profile.is_registered = true;

    emit!(RegisterEvent{
      whitelist_signature
    });

    Ok(())
  }

  pub fn redeem_by_sol(
    ctx: Context<RedeemBySolContext>,
    amount: u64,
  ) -> Result<()> {

    let user = &ctx.accounts.user;
    let launchpad = &ctx.accounts.launchpad;
    let launchpad_signer = &ctx.accounts.launchpad_signer;
    let user_profile = &ctx.accounts.user_profile;
    let user_token_account = &ctx.accounts.user_token_account;
    let launchpad_token_account = &ctx.accounts.launchpad_token_account;
    let fee_owner = &ctx.accounts.fee_owner;
    let clock = Clock::get().unwrap();

    require!(launchpad.is_active, ErrorCode::LaunchpadInactive);
    require!(launchpad.total_sold + amount <= launchpad.total_limit, ErrorCode::ReachLimitSold);
    require!(launchpad.amount_sold_in_sol + amount <= launchpad.amount_limit_in_sol, ErrorCode::MaxAmountReached);
    require!(launchpad.price_n > 0u64, ErrorCode::NotAllowed);
    require!(user_profile.is_registered, ErrorCode::Unauthorized);
    require!(
      clock.unix_timestamp >= launchpad.redeem_start_timestamp && clock.unix_timestamp <= launchpad.redeem_end_timestamp,
      ErrorCode::NotInTimeframe,
    );
    require!(
      launchpad.min_per_tx == 0u64 || amount >= launchpad.min_per_tx,
      ErrorCode::MinAmountNotSatisfied,
    );
    let redeemed_amount = user_profile.redeemed_token.checked_add(amount).unwrap();
    require!(
      launchpad.max_per_user == 0u64 || redeemed_amount <= launchpad.max_per_user,
      ErrorCode::MaxAmountReached,
    );

    let launchpad = &mut ctx.accounts.launchpad;

    let amount_sol = calculate_sub_total(amount, launchpad.price_n, launchpad.price_d)
      .unwrap();
    // Transfer lamports
    transfer_lamport(
      &user.to_account_info(),
      &launchpad_signer.to_account_info(),
      amount_sol,
      &[],
    ).expect("Starship: CPI failed.");

    let user_profile = &mut ctx.accounts.user_profile;

    user_profile.redeemed_token = redeemed_amount;

    let seeds: &[&[_]] = &[
      &SIGNER_SEED_1,
      launchpad.to_account_info().key.as_ref(),
      &[launchpad.signer_nonce],
    ];

    let system_fee = calculate_system_fee(amount_sol, launchpad.protocol_fee, launchpad.sharing_fee);

    require!(system_fee < amount_sol, ErrorCode::InvalidFee);

    if system_fee > 0 {
      transfer_lamport(
        launchpad_signer,
        fee_owner,
        system_fee,
        &[seeds],
      ).expect("Starship: CPI failed");
    }

    let amount_out = calculate_out_total(amount_sol, launchpad.price_n, launchpad.price_d).unwrap();

    launchpad.total_sold += amount_out;
    launchpad.amount_sold_in_sol += amount;

    if clock.unix_timestamp < launchpad.claim_start_timestamp {
      user_profile.pending_token += amount_out;
    } else {
      // Transfer token 1
      launchpad.total_claimed += amount;
      transfer_token(
        launchpad_signer,
        &launchpad_token_account.to_account_info(),
        &user_token_account,
        amount_out,
        &[seeds],
      ).expect("Starship: CPI failed.");
    }

    emit!(RedeemBySolEvent{
      amount,
    });

    Ok(())
  }

  pub fn redeem_by_token(
    ctx: Context<RedeemByTokenContext>,
    amount: u64,
  ) -> Result<()> {

    let user = &ctx.accounts.user;
    let launchpad = &ctx.accounts.launchpad;
    let launchpad_purchase = &ctx.accounts.launchpad_purchase;
    let launchpad_signer = &ctx.accounts.launchpad_signer;
    let user_profile = &ctx.accounts.user_profile;
    let user_token0_account = &ctx.accounts.user_token0_account;
    let user_token1_account = &ctx.accounts.user_token1_account;
    let launchpad_token0_account = &ctx.accounts.launchpad_token0_account;
    let launchpad_token1_account = &ctx.accounts.launchpad_token1_account;
    let fee_owner_token0_account = &ctx.accounts.fee_owner_token0_account;
    let clock = Clock::get().unwrap();

    require!(launchpad.is_active, ErrorCode::LaunchpadInactive);
    require!(launchpad.total_sold + amount <= launchpad.total_limit, ErrorCode::ReachLimitSold);
    require!(launchpad_purchase.amount_sold_in_token + amount <= launchpad_purchase.amount_limit_in_token, ErrorCode::MaxAmountReached);
    require!(launchpad_purchase.price_n > 0u64, ErrorCode::NotAllowed);
    require!(user_profile.is_registered, ErrorCode::Unauthorized);
    require!(
      clock.unix_timestamp >= launchpad.redeem_start_timestamp && clock.unix_timestamp <= launchpad.redeem_end_timestamp,
      ErrorCode::NotInTimeframe,
    );
    require!(
      launchpad.min_per_tx == 0u64 || amount >= launchpad.min_per_tx,
      ErrorCode::MinAmountNotSatisfied,
    );
    let redeemed_amount = user_profile.redeemed_token.checked_add(amount).unwrap();
    require!(
      launchpad.max_per_user == 0u64 || redeemed_amount <= launchpad.max_per_user,
      ErrorCode::MaxAmountReached,
    );

    let launchpad = &mut ctx.accounts.launchpad;
    let launchpad_purchase = &mut ctx.accounts.launchpad_purchase;

    let amount_token0 = calculate_sub_total(amount, launchpad_purchase.price_n, launchpad_purchase.price_d)
      .unwrap();
    // Transfer token 0
    transfer_token(
      &user.to_account_info(),
      &user_token0_account.to_account_info(),
      &launchpad_token0_account.to_account_info(),
      amount_token0,
      &[],
    ).expect("Starship: CPI failed.");

    let user_profile = &mut ctx.accounts.user_profile;

    user_profile.redeemed_token = redeemed_amount;

    let seeds: &[&[_]] = &[
      &SIGNER_SEED_1,
      launchpad.to_account_info().key.as_ref(),
      &[launchpad.signer_nonce],
    ];

    let system_fee = calculate_system_fee(amount_token0, launchpad.protocol_fee, launchpad_purchase.sharing_fee);

    require!(system_fee < amount_token0, ErrorCode::InvalidFee);

    if system_fee > 0 {
      transfer_token(
        &launchpad_signer,
        &launchpad_token0_account.to_account_info(),
        &fee_owner_token0_account.to_account_info(),
        system_fee,
        &[seeds],
      ).expect("Starship: CPI failed");
    }

    let amount_out = calculate_out_total(amount_token0, launchpad_purchase.price_n, launchpad_purchase.price_d).unwrap();

    launchpad.total_sold += amount_out;
    launchpad_purchase.amount_sold_in_token += amount_out;

    if clock.unix_timestamp < launchpad.claim_start_timestamp {
      user_profile.pending_token += amount_out;
    } else {
      // Transfer token 1
      launchpad.total_claimed += amount_out;
      transfer_token(
        &launchpad_signer,
        &launchpad_token1_account.to_account_info(),
        &user_token1_account.to_account_info(),
        amount_out,
        &[seeds],
      ).expect("Starship: CPI failed.");
    }

    emit!(RedeemByTokenEvent{
      amount,
    });

    Ok(())
  }

  pub fn create_user_profile(
    ctx: Context<CreateUserProfileContext>,
    user: Pubkey,
  ) -> Result<()> {

    let launchpad = &ctx.accounts.launchpad;

    let profile = &mut ctx.accounts.user_profile;

    profile.nonce = *ctx.bumps.get("user_profile").unwrap();
    profile.launchpad = launchpad.key();
    profile.user = user;

    emit!(CreateUserProfileEvent{
      user,
    });

    Ok(())
  }

  pub fn claim_pending_token(
    ctx: Context<ClaimPendingTokenContext>,
    amount: u64
  ) -> Result<()> {
    let launchpad = &mut ctx.accounts.launchpad;
    let launchpad_signer = &ctx.accounts.launchpad_signer;
    let user_profile = &mut ctx.accounts.user_profile;
    let launchpad_token1_account = &ctx.accounts.launchpad_token1_account;
    let user_token1_account = &ctx.accounts.user_token1_account;

    let clock = Clock::get().unwrap();

    require!(clock.unix_timestamp > launchpad.claim_start_timestamp, ErrorCode::NotInTimeframe);
    require!(user_profile.pending_token <= amount, ErrorCode::InvalidInput);

    user_profile.pending_token -= amount;
    launchpad.total_claimed += amount;

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
      &[seeds],
    ).expect("Starship: CPI failed.");

    Ok(())
  }

  #[access_control(verify_owner(ctx.accounts.launchpad.owner, *ctx.accounts.owner.key))]
  pub fn withdraw_sol(
    ctx: Context<WithdrawSolContext>,
    amount: u64
  ) -> Result<()> {
    let root = &ctx.accounts.owner;
    let launchpad = &ctx.accounts.launchpad;
    let launchpad_signer = &ctx.accounts.launchpad_signer;

    let seeds: &[&[_]] = &[
      &SIGNER_SEED_1,
      launchpad.to_account_info().key.as_ref(),
      &[launchpad.signer_nonce],
    ];

    transfer_lamport(
      &launchpad_signer.to_account_info(),
      &root.to_account_info(),
      amount,
      &[seeds],
    ).expect("Starship: CPI failed.");

    emit!(WithdrawSolEvent{
      amount,
    });

    Ok(())
  }

  #[access_control(verify_owner(ctx.accounts.launchpad.owner, *ctx.accounts.owner.key))]
  pub fn withdraw_token(
    ctx: Context<WithdrawTokenContext>,
    token_mint: Pubkey,
    amount: u64
  ) -> Result<()> {
    let launchpad = &ctx.accounts.launchpad;
    let launchpad_signer = &ctx.accounts.launchpad_signer;
    let from = &mut ctx.accounts.from;
    let to = &ctx.accounts.to;

    let seeds: &[&[_]] = &[
      &SIGNER_SEED_1,
      launchpad.to_account_info().key.as_ref(),
      &[launchpad.signer_nonce],
    ];

    transfer_token(
      launchpad_signer,
      &from.to_account_info(),
      &to,
      amount,
      &[seeds],
    ).expect("Starship: CPI failed.");

    from.reload()?;

    if token_mint == launchpad.token_mint {
      let left_balance = from.amount;
      require!(left_balance >= launchpad.total_sold - launchpad.total_claimed, ErrorCode::ReachLimitWithdraw);
    }

    emit!(WithdrawTokenEvent{
      amount,
    });
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

pub fn verify_owner(expect_owner: Pubkey, user: Pubkey) -> Result<()> {
  if expect_owner != user {
    return Err(ErrorCode::Unauthorized.into());
  }
  Ok(())
}
