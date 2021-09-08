use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use solana_program::instruction::{ AccountMeta };

#[program]
mod coin98_lunapad {
  use super::*;

  pub fn init(
    ctx: Context<InitContext>,
  ) -> ProgramResult {
    msg!("Coin98Lunapad: Instruction_Init");

    let root = &ctx.accounts.root;
    let app_data = &ctx.accounts.app_data;

    if app_data.is_initialized {
      return Err(ErrorCode::LunapadInitialized.into());
    }

    let app_data = &mut ctx.accounts.app_data;

    app_data.root = *root.to_account_info().key;

    Ok(())
  }

  pub fn create_launchpad(
    ctx: Context<CreateLaunchpadContext>,
    _launchpad_path: Vec<u8>,
    _launchpad_nonce: u8,
    signer_nonce: u8,
    owner: Pubkey,
    allow_sol: bool,
    price_in_sol: u64,
    allow_token: bool,
    price_in_token: u64,
    token0_mint: Pubkey,
    token1_mint: Pubkey,
    vault: Pubkey,
    vault_signer: Pubkey,
    vault_token0: Pubkey,
    vault_token1: Pubkey,
    is_private_sale: bool,
    sale_limit_per_tx: u64,
    sale_limit_per_user: u64,
    register_start_timestamp: i64,
    register_end_timestamp: i64,
    redeem_start_timestamp: i64,
    redeem_end_timestamp: i64,
    is_finalized: bool,
  ) -> ProgramResult {
    msg!("Coin98Lunapad: Instruction_CreateLaunchpad");

    let root = &ctx.accounts.root;
    let app_data = &ctx.accounts.app_data;
    let clock = &ctx.accounts.clock;

    if app_data.root != *root.to_account_info().key {
      return Err(ErrorCode::InvalidOwner.into());
    }
    if allow_sol && price_in_sol == 0u64 {
      return Err(ErrorCode::InvalidSolPrice.into());
    }
    if allow_token && price_in_token == 0u64 {
      return Err(ErrorCode::InvalidTokenPrice.into());
    }
    if register_start_timestamp > register_end_timestamp {
      return Err(ErrorCode::InvalidRegistrationTime.into());
    }
    if clock.unix_timestamp > register_start_timestamp {
      return Err(ErrorCode::FutureTimeRequired.into());
    }
    if redeem_start_timestamp > redeem_end_timestamp {
      return Err(ErrorCode::InvalidSaleTime.into());
    }
    if redeem_start_timestamp < register_end_timestamp {
      return Err(ErrorCode::RegistrationAndSaleTimeOverlap.into());
    }

    let launchpad = &mut ctx.accounts.launchpad;

    launchpad.nonce = signer_nonce;
    launchpad.owner = owner;
    launchpad.allow_sol = allow_sol;
    launchpad.price_in_sol = price_in_sol;
    launchpad.allow_token = allow_token;
    launchpad.price_in_token = price_in_token;
    launchpad.token0_mint = token0_mint;
    launchpad.token1_mint = token1_mint;
    launchpad.vault = vault;
    launchpad.vault_signer = vault_signer;
    launchpad.vault_token0 =  vault_token0;
    launchpad.vault_token1 =  vault_token1;
    launchpad.is_private_sale = is_private_sale;
    launchpad.sale_limit_per_tx = sale_limit_per_tx;
    launchpad.sale_limit_per_user = sale_limit_per_user;
    launchpad.register_start_timestamp = register_start_timestamp;
    launchpad.register_end_timestamp = register_end_timestamp;
    launchpad.redeem_start_timestamp = redeem_start_timestamp;
    launchpad.redeem_end_timestamp = redeem_end_timestamp;
    launchpad.is_finalized = is_finalized;

    Ok(())
  }

  pub fn update_launchpad(
    ctx: Context<UpdateLaunchpadContext>,
    allow_sol: bool,
    price_in_sol: u64,
    allow_token: bool,
    price_in_token: u64,
    token0_mint: Pubkey,
    token1_mint: Pubkey,
    vault: Pubkey,
    vault_signer: Pubkey,
    vault_token0: Pubkey,
    vault_token1: Pubkey,
    is_private_sale: bool,
    sale_limit_per_tx: u64,
    sale_limit_per_user: u64,
    register_start_timestamp: i64,
    register_end_timestamp: i64,
    redeem_start_timestamp: i64,
    redeem_end_timestamp: i64,
    is_finalized: bool,
  ) -> ProgramResult {
    msg!("Coin98Lunapad: Instruction_UpdateLaunchpad");

    let owner = &ctx.accounts.owner;
    let launchpad = &ctx.accounts.launchpad;
    let clock = &ctx.accounts.clock;

    if launchpad.owner != *owner.key {
      return Err(ErrorCode::InvalidOwner.into());
    }
    if launchpad.is_finalized || clock.unix_timestamp > launchpad.register_start_timestamp {
      return Err(ErrorCode::LaunchpadFinalized.into());
    }
    if allow_sol && price_in_sol == 0u64 {
      return Err(ErrorCode::InvalidSolPrice.into());
    }
    if allow_token && price_in_token == 0u64 {
      return Err(ErrorCode::InvalidTokenPrice.into());
    }
    if clock.unix_timestamp > register_start_timestamp {
      return Err(ErrorCode::FutureTimeRequired.into());
    }
    if register_start_timestamp > register_end_timestamp {
      return Err(ErrorCode::InvalidRegistrationTime.into());
    }
    if redeem_start_timestamp > redeem_end_timestamp {
      return Err(ErrorCode::InvalidSaleTime.into());
    }
    if redeem_start_timestamp < register_end_timestamp {
      return Err(ErrorCode::RegistrationAndSaleTimeOverlap.into());
    }

    let launchpad = &mut ctx.accounts.launchpad;

    launchpad.allow_sol = allow_sol;
    launchpad.price_in_sol = price_in_sol;
    launchpad.allow_token = allow_token;
    launchpad.price_in_token = price_in_token;
    launchpad.token0_mint = token0_mint;
    launchpad.token1_mint = token1_mint;
    launchpad.vault = vault;
    launchpad.vault_signer = vault_signer;
    launchpad.vault_token0 =  vault_token0;
    launchpad.vault_token1 =  vault_token1;
    launchpad.is_private_sale = is_private_sale;
    launchpad.sale_limit_per_tx = sale_limit_per_tx;
    launchpad.sale_limit_per_user = sale_limit_per_user;
    launchpad.register_start_timestamp = register_start_timestamp;
    launchpad.register_end_timestamp = register_end_timestamp;
    launchpad.redeem_start_timestamp = redeem_start_timestamp;
    launchpad.redeem_end_timestamp = redeem_end_timestamp;
    launchpad.is_finalized = is_finalized;

    Ok(())
  }

  pub fn register(
    ctx: Context<RegisterContext>,
  ) -> ProgramResult {
    msg!("Coin98Lunapad: Instruction_Register");

    let user = &ctx.accounts.user;
    let launchpad = &ctx.accounts.launchpad;
    let global_profile = &ctx.accounts.global_profile;
    let local_profile = &ctx.accounts.local_profile;
    let clock = &ctx.accounts.clock;

    if global_profile.user != *user.to_account_info().key {
      return Err(ErrorCode::InvalidUser.into());
    }
    if global_profile.is_blacklisted {
      return Err(ErrorCode::Blacklisted.into());
    }
    if local_profile.user != *user.to_account_info().key {
      return Err(ErrorCode::InvalidUser.into());
    }
    if local_profile.launchpad != *launchpad.to_account_info().key {
      return Err(ErrorCode::InvalidLanchpad.into());
    }
    if launchpad.is_private_sale && !local_profile.is_whitelisted {
      return Err(ErrorCode::NotWhitelisted.into());
    }
    if clock.unix_timestamp < launchpad.register_start_timestamp || clock.unix_timestamp > launchpad.register_end_timestamp {
      return Err(ErrorCode::InvalidSaleTime.into());
    }

    let local_profile = &mut ctx.accounts.local_profile;
    local_profile.is_registered = true;

    Ok(())
  }

  pub fn redeem_by_sol(
    ctx: Context<RedeemBySolContext>,
    amount: u64,
  ) -> ProgramResult {
    msg!("Coin98Lunapad: Instruction_RedeemBySOL");

    let user = &ctx.accounts.user;
    let launchpad = &ctx.accounts.launchpad;
    let launchpad_signer = &ctx.accounts.launchpad_signer;
    let global_profile = &ctx.accounts.global_profile;
    let local_profile = &ctx.accounts.local_profile;
    let user_token1 = &ctx.accounts.user_token1;
    let vault = &ctx.accounts.vault;
    let vault_signer = &ctx.accounts.vault_signer;
    let vault_token1 = &ctx.accounts.vault_token1;
    let clock = &ctx.accounts.clock;
    let vault_program = &ctx.accounts.vault_program;
    let token_program = &ctx.accounts.token_program;

    if global_profile.user != *user.to_account_info().key {
      return Err(ErrorCode::InvalidUser.into());
    }
    if global_profile.is_blacklisted {
      return Err(ErrorCode::Blacklisted.into());
    }
    if local_profile.user != *user.to_account_info().key {
      return Err(ErrorCode::InvalidUser.into());
    }
    if local_profile.launchpad != *launchpad.to_account_info().key {
      return Err(ErrorCode::InvalidLanchpad.into());
    }
    if !launchpad.allow_sol {
      return Err(ErrorCode::RedeemBySolNotAllowed.into());
    }
    if launchpad.is_private_sale && !local_profile.is_whitelisted {
      return Err(ErrorCode::NotWhitelisted.into());
    }
    if !local_profile.is_registered {
      return Err(ErrorCode::NotRegistered.into());
    }
    if clock.unix_timestamp < launchpad.redeem_start_timestamp || clock.unix_timestamp > launchpad.redeem_end_timestamp {
      return Err(ErrorCode::InvalidSaleTime.into());
    }

    let amount_sol = amount * launchpad.price_in_sol;
    let instruction = &solana_program::system_instruction::transfer(user.key, vault_signer.key, amount_sol);
    let result = solana_program::program::invoke(&instruction, &[
      user.clone(), vault_signer.clone()
    ]);
    if result.is_err() {
      return Err(ErrorCode::TransactionFailed.into());
    }

    let local_profile = &mut ctx.accounts.local_profile;

    local_profile.redeemed_token += amount;

    let withdraw_params = TransferTokenParams {
      amount: amount,
    };
    let mut withdraw_data: Vec<u8> = Vec::new();
    withdraw_data.extend_from_slice(&[136, 235, 181, 5, 101, 109, 57, 81]);
    withdraw_data.extend_from_slice(&withdraw_params.try_to_vec().unwrap());

    let instruction = solana_program::instruction::Instruction {
      program_id: *vault_program.key,
      accounts: vec![
        solana_program::instruction::AccountMeta::new_readonly(*vault.key, false),
        solana_program::instruction::AccountMeta::new_readonly(*vault_signer.key, false),
        solana_program::instruction::AccountMeta::new_readonly(*launchpad_signer.key, true),
        solana_program::instruction::AccountMeta::new(*vault_token1.key, false),
        solana_program::instruction::AccountMeta::new(*user_token1.key, false),
        solana_program::instruction::AccountMeta::new_readonly(*token_program.key, false),
      ],
      data: withdraw_data,
    };
    let seeds: &[&[_]] = &[
      &[2, 151, 229, 53, 244,  77, 229,  7],
      launchpad.to_account_info().key.as_ref(),
      &[launchpad.nonce],
    ];
    let result = solana_program::program::invoke_signed(&instruction, &[
      vault.clone(),
      vault_signer.clone(),
      launchpad_signer.clone(),
      vault_token1.clone(),
      user_token1.clone(),
      token_program.clone(),
    ], &[&seeds]);
    if result.is_err() {
      return Err(ErrorCode::TransactionFailed.into());
    }

    Ok(())
  }

  pub fn redeem_by_token(
    ctx: Context<RedeemByTokenContext>,
    amount: u64,
  ) -> ProgramResult {
    msg!("Coin98Lunapad: Instruction_RedeemByToken");

    let user = &ctx.accounts.user;
    let launchpad = &ctx.accounts.launchpad;
    let launchpad_signer = &ctx.accounts.launchpad_signer;
    let global_profile = &ctx.accounts.global_profile;
    let local_profile = &ctx.accounts.local_profile;
    let user_token0 = &ctx.accounts.user_token1;
    let user_token1 = &ctx.accounts.user_token1;
    let vault = &ctx.accounts.vault;
    let vault_signer = &ctx.accounts.vault_signer;
    let vault_token0 = &ctx.accounts.vault_token1;
    let vault_token1 = &ctx.accounts.vault_token1;
    let clock = &ctx.accounts.clock;
    let vault_program = &ctx.accounts.vault_program;
    let token_program = &ctx.accounts.token_program;

    if global_profile.user != *user.to_account_info().key {
      return Err(ErrorCode::InvalidUser.into());
    }
    if global_profile.is_blacklisted {
      return Err(ErrorCode::Blacklisted.into());
    }
    if local_profile.user != *user.to_account_info().key {
      return Err(ErrorCode::InvalidUser.into());
    }
    if local_profile.launchpad != *launchpad.to_account_info().key {
      return Err(ErrorCode::InvalidLanchpad.into());
    }
    if !launchpad.allow_token {
      return Err(ErrorCode::RedeemByTokenNotAllowed.into());
    }
    if launchpad.is_private_sale && !local_profile.is_whitelisted {
      return Err(ErrorCode::NotWhitelisted.into());
    }
    if clock.unix_timestamp < launchpad.redeem_start_timestamp || clock.unix_timestamp > launchpad.redeem_end_timestamp {
      return Err(ErrorCode::InvalidSaleTime.into());
    }

    let amount_token0 = amount * launchpad.price_in_token;
    let transfer_params = TransferTokenParams {
      amount: amount_token0,
    };
    let mut transfer_data: Vec<u8> = Vec::new();
    transfer_data.extend_from_slice(&[3]);
    transfer_data.extend_from_slice(&transfer_params.try_to_vec().unwrap());
    let instruction = solana_program::instruction::Instruction {
      program_id: *token_program.key,
      accounts: vec![
        solana_program::instruction::AccountMeta::new(*user_token0.key, false),
        solana_program::instruction::AccountMeta::new(*vault_token0.key, false),
        solana_program::instruction::AccountMeta::new_readonly(*user.key, true),
      ],
      data: transfer_data,
    };
    let result = solana_program::program::invoke(&instruction, &[
      user_token0.clone(),
      vault_token0.clone(),
      user.clone(),
    ]);
    if result.is_err() {
      return Err(ErrorCode::TransactionFailed.into());
    }

    let local_profile = &mut ctx.accounts.local_profile;

    local_profile.redeemed_token += amount;

    let withdraw_params = TransferTokenParams {
      amount: amount,
    };
    let mut withdraw_data: Vec<u8> = Vec::new();
    withdraw_data.extend_from_slice(&[136, 235, 181, 5, 101, 109, 57, 81]);
    withdraw_data.extend_from_slice(&withdraw_params.try_to_vec().unwrap());

    let instruction = solana_program::instruction::Instruction {
      program_id: *vault_program.key,
      accounts: vec![
        solana_program::instruction::AccountMeta::new_readonly(*vault.key, false),
        solana_program::instruction::AccountMeta::new_readonly(*vault_signer.key, false),
        solana_program::instruction::AccountMeta::new_readonly(*launchpad_signer.key, true),
        solana_program::instruction::AccountMeta::new(*vault_token1.key, false),
        solana_program::instruction::AccountMeta::new(*user_token1.key, false),
        solana_program::instruction::AccountMeta::new_readonly(*token_program.key, false),
      ],
      data: withdraw_data,
    };
    let seeds: &[&[_]] = &[
      &[2, 151, 229, 53, 244,  77, 229,  7],
      launchpad.to_account_info().key.as_ref(),
      &[launchpad.nonce],
    ];
    let result = solana_program::program::invoke_signed(&instruction, &[
      vault.clone(),
      vault_signer.clone(),
      launchpad_signer.clone(),
      vault_token1.clone(),
      user_token1.clone(),
      token_program.clone(),
    ], &[&seeds]);
    if result.is_err() {
      return Err(ErrorCode::TransactionFailed.into());
    }

    Ok(())
  }

  pub fn set_whitelist_internal(
    ctx: Context<SetWhitelistInternalContext>,
    is_whitelisted: bool,
  ) -> ProgramResult {
    msg!("Coin98Lunapad: Instruction_SetWhitelistInternal");

    let launchpad = &ctx.accounts.launchpad;
    let launchpad_signer = &ctx.accounts.launchpad_signer;
    let user = &ctx.accounts.user;
    let profile = &ctx.accounts.local_profile;

    let signer_address = Pubkey::create_program_address(
      &[
        &[2, 151, 229, 53, 244,  77, 229,  7],
        launchpad.to_account_info().key.as_ref(),
        &[launchpad.nonce]
      ],
      &ctx.program_id,
    ).unwrap();

    if signer_address != *launchpad_signer.key {
      return Err(ErrorCode::InvalidOwner.into());
    }

    if profile.user != *user.key {
      return Err(ErrorCode::InvalidUser.into());
    }

    let profile = &mut ctx.accounts.local_profile;

    profile.is_whitelisted = is_whitelisted;

    Ok(())
  }

  pub fn set_whitelists(
    ctx: Context<SetWhitelistsContext>,
    is_whitelisted: bool,
  ) -> ProgramResult {
    msg!("Coin98Lunapad: Instruction_SetWhitelists");

    let owner = &ctx.accounts.owner;
    let launchpad = &ctx.accounts.launchpad;
    let clock = &ctx.accounts.clock;

    if launchpad.owner != *owner.key {
      return Err(ErrorCode::InvalidOwner.into());
    }
    if clock.unix_timestamp > launchpad.redeem_start_timestamp {
      return Err(ErrorCode::InvalidRegistrationTime.into());
    }

    let signer_address = Pubkey::create_program_address(
      &[
        &[2, 151, 229, 53, 244,  77, 229,  7],
        launchpad.to_account_info().key.as_ref(),
        &[launchpad.nonce]
      ],
      &ctx.program_id,
    ).unwrap();
    let signer_seeds: &[&[_]] = &[
      &[2, 151, 229, 53, 244,  77, 229,  7],
      launchpad.to_account_info().key.as_ref(),
      &[launchpad.nonce],
    ];

    let mut set_whitelist_data: Vec<u8> = Vec::new();
    set_whitelist_data.extend_from_slice(&[237, 84, 123, 225, 100, 205, 44, 246]);
    if is_whitelisted {
      set_whitelist_data.extend_from_slice(&[1]);
    }
    else {
      set_whitelist_data.extend_from_slice(&[0]);
    }

    let accounts = &ctx.remaining_accounts;

    let mut i = 4;
    while i < accounts.len() {
      let instruction = solana_program::instruction::Instruction {
        program_id: *ctx.program_id,
        accounts: vec![
          AccountMeta::new_readonly(*launchpad.to_account_info().key, false),
          AccountMeta::new_readonly(signer_address, true),
          AccountMeta::new_readonly(*accounts[i].to_account_info().key, false),
          AccountMeta::new(*accounts[i+1].to_account_info().key, false),
          AccountMeta::new_readonly(*clock.to_account_info().key, false),
        ],
        data: set_whitelist_data.to_vec(),
      };
      let result = solana_program::program::invoke_signed(&instruction, &[
        accounts[0].clone(),
        accounts[1].clone(),
        accounts[i].clone(),
        accounts[i+1].clone(),
        accounts[2].clone(),
      ], &[&signer_seeds]);
      if result.is_err() {
        return Err(ErrorCode::TransactionFailed.into());
      }
      i = i + 2;
    }

    Ok(())
  }

  pub fn set_blacklist(
    ctx: Context<SetBlacklistContext>,
    is_blacklisted: bool,
  ) -> ProgramResult {
    msg!("Coin98Lunapad: Instruction_SetBlacklist");

    let root = &ctx.accounts.root;
    let app_data = &ctx.accounts.app_data;
    let user = &ctx.accounts.user;
    let profile = &ctx.accounts.global_profile;

    if app_data.root != *root.to_account_info().key {
      return Err(ErrorCode::InvalidOwner.into());
    }
    if profile.user != *user.key {
      return Err(ErrorCode::InvalidUser.into());
    }

    let profile = &mut ctx.accounts.global_profile;

    profile.is_blacklisted = is_blacklisted;

    Ok(())
  }

  pub fn transfer_root(ctx: Context<TransferRootContext>,
    new_root: Pubkey,
  ) -> ProgramResult {
    msg!("Coin98Lunapad: Instruction_TransferRoot");

    let root = &ctx.accounts.root;
    let app_data = &ctx.accounts.app_data;

    if app_data.root != *root.key {
      return Err(ErrorCode::InvalidOwner.into());
    }

    let app_data = &mut ctx.accounts.app_data;

    app_data.new_root = new_root;

    Ok(())
  }

  pub fn accept_root(ctx: Context<AcceptRootContext>,
  ) -> ProgramResult {
    msg!("Coin98Lunapad: Instruction_AcceptRoot");

    let new_root = &ctx.accounts.new_root;
    let app_data = &ctx.accounts.app_data;

    if app_data.new_root != *new_root.key {
      return Err(ErrorCode::InvalidNewOwner.into());
    }

    let app_data = &mut ctx.accounts.app_data;

    app_data.root = app_data.new_root;
    app_data.new_root = solana_program::system_program::ID;

    Ok(())
  }

  pub fn transfer_ownership(ctx: Context<TransferOwnershipContext>,
    new_owner: Pubkey,
  ) -> ProgramResult {
    msg!("Coin98Lunapad: Instruction_TransferOwnership");

    let owner = &ctx.accounts.owner;
    let launchpad = &ctx.accounts.launchpad;

    if launchpad.owner != *owner.key {
      return Err(ErrorCode::InvalidOwner.into());
    }

    let launchpad = &mut ctx.accounts.launchpad;

    launchpad.new_owner = new_owner;

    Ok(())
  }

  pub fn accept_ownership(ctx: Context<AcceptOwnershipContext>,
  ) -> ProgramResult {
    msg!("Coin98Lunapad: Instruction_AcceptOwnership");

    let new_owner = &ctx.accounts.new_owner;
    let launchpad = &ctx.accounts.launchpad;

    if launchpad.new_owner != *new_owner.key {
      return Err(ErrorCode::InvalidNewOwner.into());
    }

    let launchpad = &mut ctx.accounts.launchpad;

    launchpad.owner = launchpad.new_owner;
    launchpad.new_owner = solana_program::system_program::ID;

    Ok(())
  }

  pub fn create_app_data(
    _ctx: Context<CreateAppDataContext>,
    _nonce: u8,
  ) -> ProgramResult {
    msg!("Coin98Lunapad: Instruction_CreateAppData");

    Ok(())
  }

  pub fn create_global_profile(
    ctx: Context<CreateGlobalProfileContext>,
    _nonce: u8,
  ) -> ProgramResult {
    msg!("Coin98Lunapad: Instruction_CreateGlobalProfile");

    let user = &ctx.accounts.user;

    let profile = &mut ctx.accounts.global_profile;

    profile.user = *user.key;

    Ok(())
  }

  pub fn create_local_profile(
    ctx: Context<CreateLocalProfileContext>,
    _nonce: u8,
  ) -> ProgramResult {
    msg!("Coin98Lunapad: Instruction_CreateLocalProfile");

    let launchpad = &ctx.accounts.launchpad;
    let user = &ctx.accounts.user;

    let profile = &mut ctx.accounts.local_profile;

    profile.launchpad = *launchpad.key;
    profile.user = *user.key;

    Ok(())
  }
}

#[derive(Accounts)]
pub struct InitContext<'info> {

  #[account(signer)]
  pub root: AccountInfo<'info>,

  #[account(mut)]
  pub app_data: ProgramAccount<'info, AppData>,
}

#[derive(Accounts)]
#[instruction(launchpad_path: Vec<u8>, launchpad_nonce: u8, _signer_nonce: u8)]
pub struct CreateLaunchpadContext<'info> {

  #[account(signer)]
  pub root: AccountInfo<'info>,

  pub app_data: ProgramAccount<'info, AppData>,

  #[account(init, seeds = [
    &[8, 201, 24, 140, 93, 100, 30, 148],
    &*launchpad_path,
    &[launchpad_nonce]
  ], payer = root, space = 340)]
  pub launchpad: ProgramAccount<'info, Launchpad>,

  pub rent: Sysvar<'info, Rent>,

  pub clock: Sysvar<'info, Clock>,

  pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateLaunchpadContext<'info> {

  #[account(signer)]
  pub owner: AccountInfo<'info>,

  #[account(mut)]
  pub launchpad: ProgramAccount<'info, Launchpad>,

  pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct RegisterContext<'info> {

  pub launchpad: ProgramAccount<'info, Launchpad>,

  #[account(signer)]
  pub user: AccountInfo<'info>,

  pub global_profile: ProgramAccount<'info, GlobalProfile>,

  #[account(mut)]
  pub local_profile: ProgramAccount<'info, LocalProfile>,

  pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct RedeemBySolContext<'info> {

  pub launchpad: ProgramAccount<'info, Launchpad>,

  #[account(seeds = [
    &[2, 151, 229, 53, 244,  77, 229,  7],
    launchpad.to_account_info().key.as_ref(),
    &[launchpad.nonce],
  ])]
  pub launchpad_signer: AccountInfo<'info>,

  #[account(signer)]
  pub user: AccountInfo<'info>,

  pub global_profile: ProgramAccount<'info, GlobalProfile>,

  #[account(mut)]
  pub local_profile: ProgramAccount<'info, LocalProfile>,

  #[account(mut)]
  pub user_token1: AccountInfo<'info>,

  pub vault: AccountInfo<'info>,

  #[account(mut)]
  pub vault_signer: AccountInfo<'info>,

  #[account(mut)]
  pub vault_token1: AccountInfo<'info>,

  pub clock: Sysvar<'info, Clock>,

  pub vault_program: AccountInfo<'info>,

  pub system_program: AccountInfo<'info>,

  pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RedeemByTokenContext<'info> {

  pub launchpad: ProgramAccount<'info, Launchpad>,

  #[account(seeds = [
    &[2, 151, 229, 53, 244,  77, 229,  7],
    launchpad.to_account_info().key.as_ref(),
    &[launchpad.nonce],
  ])]
  pub launchpad_signer: AccountInfo<'info>,

  #[account(signer)]
  pub user: AccountInfo<'info>,

  pub global_profile: ProgramAccount<'info, GlobalProfile>,

  #[account(mut)]
  pub local_profile: ProgramAccount<'info, LocalProfile>,

  #[account(mut)]
  pub user_token0: AccountInfo<'info>,

  #[account(mut)]
  pub user_token1: AccountInfo<'info>,

  pub vault: AccountInfo<'info>,

  pub vault_signer: AccountInfo<'info>,

  #[account(mut)]
  pub vault_token0: AccountInfo<'info>,

  #[account(mut)]
  pub vault_token1: AccountInfo<'info>,

  pub clock: Sysvar<'info, Clock>,

  pub vault_program: AccountInfo<'info>,

  pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetWhitelistInternalContext<'info> {

  pub launchpad: ProgramAccount<'info, Launchpad>,

  #[account(signer)]
  pub launchpad_signer: AccountInfo<'info>,

  pub user: AccountInfo<'info>,

  #[account(mut)]
  pub local_profile: ProgramAccount<'info, LocalProfile>,

  pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct SetWhitelistsContext<'info> {

  #[account(signer)]
  pub owner: AccountInfo<'info>,

  pub launchpad: ProgramAccount<'info, Launchpad>,

  pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct SetBlacklistContext<'info> {

  #[account(signer)]
  pub root: AccountInfo<'info>,

  pub app_data: ProgramAccount<'info, AppData>,

  pub user: AccountInfo<'info>,

  #[account(mut)]
  pub global_profile: ProgramAccount<'info, GlobalProfile>,
}

#[derive(Accounts)]
pub struct TransferRootContext<'info> {

  #[account(signer)]
  pub root: AccountInfo<'info>,

  #[account(mut)]
  pub app_data: ProgramAccount<'info, AppData>,
}

#[derive(Accounts)]
pub struct AcceptRootContext<'info> {

  #[account(signer)]
  pub new_root: AccountInfo<'info>,

  #[account(mut)]
  pub app_data: ProgramAccount<'info, AppData>,
}

#[derive(Accounts)]
pub struct TransferOwnershipContext<'info> {

  #[account(signer)]
  pub owner: AccountInfo<'info>,

  #[account(mut)]
  pub launchpad: ProgramAccount<'info, Launchpad>,
}

#[derive(Accounts)]
pub struct AcceptOwnershipContext<'info> {

  #[account(signer)]
  pub new_owner: AccountInfo<'info>,

  #[account(mut)]
  pub launchpad: ProgramAccount<'info, Launchpad>,
}

#[derive(Accounts)]
#[instruction(app_data_nonce: u8)]
pub struct CreateAppDataContext<'info> {

  #[account(signer)]
  pub payer: AccountInfo<'info>,

  #[account(init, seeds = [
    &[15, 81, 173, 106, 105, 203, 253, 99],
    Pubkey::new(&[32, 40, 118, 173, 164, 46, 192, 86, 236, 196, 165, 90, 92, 121, 96, 70, 199, 93, 172, 52, 204, 122, 54, 130, 84, 73, 55, 238, 129, 185, 214, 226]).as_ref(),
    &[app_data_nonce]
  ], payer = payer, space = 81)]
  pub global_profile: ProgramAccount<'info, AppData>,

  pub rent: Sysvar<'info, Rent>,

  pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(profile_nonce: u8)]
pub struct CreateGlobalProfileContext<'info> {

  #[account(signer)]
  pub payer: AccountInfo<'info>,

  pub user: AccountInfo<'info>,

  #[account(init, seeds = [
    &[139, 126, 195, 157, 204, 134, 142, 146],
    &[32, 40, 118, 173, 164, 46, 192, 86],
    user.key.as_ref(),
    &[profile_nonce]
  ], payer = payer, space = 49)]
  pub global_profile: ProgramAccount<'info, GlobalProfile>,

  pub rent: Sysvar<'info, Rent>,

  pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(profile_nonce: u8)]
pub struct CreateLocalProfileContext<'info> {

  #[account(signer)]
  pub payer: AccountInfo<'info>,

  pub launchpad: AccountInfo<'info>,

  pub user: AccountInfo<'info>,

  #[account(init, seeds = [
    &[133, 177, 201, 78, 13, 152, 198, 180],
    launchpad.key.as_ref(),
    user.key.as_ref(),
    &[profile_nonce]
  ], payer = payer, space = 90)]
  pub local_profile: ProgramAccount<'info, LocalProfile>,

  pub rent: Sysvar<'info, Rent>,

  pub system_program: AccountInfo<'info>,
}

#[associated]
#[derive(Default)]
pub struct Launchpad {
  pub nonce: u8,
  pub owner: Pubkey,
  pub new_owner: Pubkey,
  pub allow_sol: bool,
  pub price_in_sol: u64,
  pub allow_token: bool,
  pub price_in_token: u64,
  pub token0_mint: Pubkey,
  pub token1_mint: Pubkey,
  pub vault: Pubkey,
  pub vault_signer: Pubkey,
  pub vault_token0: Pubkey,
  pub vault_token1: Pubkey,
  pub is_private_sale: bool,
  pub sale_limit_per_tx: u64,
  pub sale_limit_per_user: u64,
  pub register_start_timestamp: i64,
  pub register_end_timestamp: i64,
  pub redeem_start_timestamp: i64,
  pub redeem_end_timestamp: i64,
  pub is_finalized: bool,
}

#[associated]
#[derive(Default)]
pub struct AppData {
  pub root: Pubkey,
  pub new_root: Pubkey,
  pub is_initialized: bool,
}

#[associated]
#[derive(Default)]
pub struct GlobalProfile {
  pub user: Pubkey,
  pub is_blacklisted: bool,
}

#[associated]
#[derive(Default)]
pub struct LocalProfile {
  pub launchpad: Pubkey,
  pub user: Pubkey,
  pub is_whitelisted: bool,
  pub is_registered: bool,
  pub redeemed_token: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default)]
pub struct TransferTokenParams {
  pub amount: u64,
}

#[error]
pub enum ErrorCode {

  #[msg("Coin98Lunapad: Forbidden.")]
  Blacklisted,

  #[msg("Coin98Lunapad: Time must be set in the future.")]
  FutureTimeRequired,

  #[msg("Coin98Lunapad: Invalid launchpad.")]
  InvalidLanchpad,

  #[msg("Coin98Lunapad: Not an owner.")]
  InvalidOwner,

  #[msg("Coin98Lunapad: Not new owner.")]
  InvalidNewOwner,

  #[msg("Coin98Lunapad: Invalid registration time range.")]
  InvalidRegistrationTime,

  #[msg("Coin98Lunapad: Invalid sale time range.")]
  InvalidSaleTime,

  #[msg("Coin98Lunapad: Invalid SOL price.")]
  InvalidSolPrice,

  #[msg("Coin98Lunapad: Invalid token price.")]
  InvalidTokenPrice,

  #[msg("Coin98Lunapad: Invalid user.")]
  InvalidUser,

  #[msg("Coin98Lunapad: Lunapad is already initialized.")]
  LunapadInitialized,

  #[msg("Coin98Lunapad: Launchpad setting is finalized.")]
  LaunchpadFinalized,

  #[msg("Coin98Lunapad: Only allowed during registration time.")]
  NotInRegistrationTime,

  #[msg("Coin98Lunapad: Only allowed during sale time.")]
  NotItSaleTime,

  #[msg("Coin98Lunapad: Not registered.")]
  NotRegistered,

  #[msg("Coin98Lunapad: Not allowed.")]
  NotWhitelisted,

  #[msg("Coin98Lunapad: Registration and sale time overlap.")]
  RegistrationAndSaleTimeOverlap,

  #[msg("Coin98Lunapad: Redeem by SOL not allowed.")]
  RedeemBySolNotAllowed,

  #[msg("Coin98Lunapad: Redeem by token not allowed.")]
  RedeemByTokenNotAllowed,

  #[msg("Coin98Lunapad: Transaction failed.")]
  TransactionFailed,
}
