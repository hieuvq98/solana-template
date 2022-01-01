pub mod constants;

use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_lang::solana_program::keccak::{ hash, hashv };
use std::convert::TryInto;

declare_id!("SS4VMP9wmqQdehu7Uc6g1Ymsx4BCVVghKp4wRmmy1jj");

#[program]
mod coin98_starship {
  use super::*;

  pub fn create_launchpad(
    ctx: Context<CreateLaunchpadContext>,
    _launchpad_path: Vec<u8>,
    _launchpad_nonce: u8,
    signer_nonce: u8,
  ) -> ProgramResult {
    msg!("Coin98Starship: Instruction_CreateLaunchpad");

    let root = &ctx.accounts.root;

    if !verify_owner(root.key) {
      return Err(ErrorCode::InvalidOwner.into());
    }

    let launchpad = &mut ctx.accounts.launchpad;

    launchpad.nonce = signer_nonce;

    Ok(())
  }

  pub fn set_launchpad(
    ctx: Context<SetLaunchpadContext>,
    price_in_sol: u64,
    price_in_token: u64,
    token_program_id: Pubkey,
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
  ) -> ProgramResult {
    msg!("Coin98Starship: Instruction_SetLaunchpad");

    let root = &ctx.accounts.root;
    let clock = &ctx.accounts.clock;

    if !verify_owner(root.key) {
      return Err(ErrorCode::InvalidOwner.into());
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

    launchpad.price_in_sol = price_in_sol;
    launchpad.price_in_token = price_in_token;
    launchpad.token_program_id = token_program_id;
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

  pub fn set_launchpad_status(
    ctx: Context<SetLaunchpadStatusContext>,
    is_active: bool,
  ) -> ProgramResult {
    msg!("Coin98Starship: Instruction_SetLaunchpadStatus");

    let root = &ctx.accounts.root;

    if !verify_owner(root.key) {
      return Err(ErrorCode::InvalidOwner.into());
    }

    let launchpad = &mut ctx.accounts.launchpad;

    launchpad.is_active = is_active;

    Ok(())
  }

  pub fn register(
    ctx: Context<RegisterContext>,
    index: u32,
    proofs: Vec<[u8; 32]>,
  ) -> ProgramResult {
    msg!("Coin98Starship: Instruction_Register");

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
    if clock.unix_timestamp < launchpad.register_start_timestamp || clock.unix_timestamp > launchpad.register_end_timestamp {
      return Err(ErrorCode::InvalidRegistrationTime.into());
    }
    if launchpad.is_private_sale {
      let whitelist = WhitelistParams {
        index: index,
        address: *user.key
      };
      let whitelist_data = whitelist.try_to_vec().unwrap();
      let leaf = hash(&whitelist_data[..]);
      let root: [u8; 32] = launchpad.private_sale_signature.clone().try_into().unwrap();
      if !verify_proof(proofs, root, leaf.to_bytes()) {
        return Err(ErrorCode::NotWhitelisted.into());
      }
    }

    let local_profile = &mut ctx.accounts.local_profile;
    local_profile.is_registered = true;

    Ok(())
  }

  pub fn redeem_by_sol(
    ctx: Context<RedeemBySolContext>,
    amount: u64,
  ) -> ProgramResult {
    msg!("Coin98Starship: Instruction_RedeemBySOL");

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
    if launchpad.price_in_sol == 0u64 {
      return Err(ErrorCode::RedeemBySolNotAllowed.into());
    }
    if !local_profile.is_registered {
      return Err(ErrorCode::NotRegistered.into());
    }
    if clock.unix_timestamp < launchpad.redeem_start_timestamp || clock.unix_timestamp > launchpad.redeem_end_timestamp {
      return Err(ErrorCode::NotInSaleTime.into());
    }
    if *vault_program.key != launchpad.vault_program_id {
      return Err(ErrorCode::InvalidVaultProgramId.into());
    }
    if *vault.key != launchpad.vault {
      return Err(ErrorCode::InvalidVault.into());
    }
    if *token_program.key != launchpad.token_program_id {
      return Err(ErrorCode::InvalidVaultProgramId.into());
    }
    if *vault_signer.key != launchpad.vault_signer {
      return Err(ErrorCode::InvalidVaultSigner.into());
    }
    if *vault_token1.key != launchpad.vault_token1 {
      return Err(ErrorCode::InvalidVaultToken1.into());
    }
    if launchpad.min_per_tx > 0 && amount < launchpad.min_per_tx {
      return Err(ErrorCode::MinAmountNotSatisfied.into());
    }
    let redeemed_amount = local_profile.redeemed_token.checked_add(amount).unwrap();
    if launchpad.max_per_user > 0 && redeemed_amount > launchpad.max_per_user {
      return Err(ErrorCode::MaxAmountReached.into());
    }

    let amount_sol = amount.checked_mul(launchpad.price_in_sol).unwrap();
    let instruction = &solana_program::system_instruction::transfer(user.key, vault_signer.key, amount_sol);
    let result = solana_program::program::invoke(&instruction, &[
      user.clone(), vault_signer.clone()
    ]);
    if result.is_err() {
      return Err(ErrorCode::TransactionFailed.into());
    }

    let local_profile = &mut ctx.accounts.local_profile;

    local_profile.redeemed_token = redeemed_amount;

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
    msg!("Coin98Starship: Instruction_RedeemByToken");

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
    if launchpad.price_in_token == 0u64 {
      return Err(ErrorCode::RedeemByTokenNotAllowed.into());
    }
    if clock.unix_timestamp < launchpad.redeem_start_timestamp || clock.unix_timestamp > launchpad.redeem_end_timestamp {
      return Err(ErrorCode::NotInSaleTime.into());
    }
    if *vault_program.key != launchpad.vault_program_id {
      return Err(ErrorCode::InvalidVaultProgramId.into());
    }
    if *token_program.key != launchpad.token_program_id {
      return Err(ErrorCode::InvalidVaultProgramId.into());
    }
    if *vault.key != launchpad.vault {
      return Err(ErrorCode::InvalidVault.into());
    }
    if *vault_signer.key != launchpad.vault_signer {
      return Err(ErrorCode::InvalidVaultSigner.into());
    }
    if *vault_token0.key != launchpad.vault_token0 {
      return Err(ErrorCode::InvalidVaultToken0.into());
    }
    if *vault_token1.key != launchpad.vault_token1 {
      return Err(ErrorCode::InvalidVaultToken1.into());
    }
    if launchpad.min_per_tx > 0 && amount < launchpad.min_per_tx {
      return Err(ErrorCode::MinAmountNotSatisfied.into());
    }
    let redeemed_amount = local_profile.redeemed_token.checked_add(amount).unwrap();
    if launchpad.max_per_user > 0 && redeemed_amount > launchpad.max_per_user {
      return Err(ErrorCode::MaxAmountReached.into());
    }

    let amount_token0 = amount.checked_mul(launchpad.price_in_token).unwrap();
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

    local_profile.redeemed_token = redeemed_amount;

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

  pub fn set_blacklist(
    ctx: Context<SetBlacklistContext>,
    user: Pubkey,
    is_blacklisted: bool,
  ) -> ProgramResult {
    msg!("Coin98Starship: Instruction_SetBlacklist");

    let root = &ctx.accounts.root;
    let profile = &ctx.accounts.global_profile;

    if !verify_owner(root.key) {
      return Err(ErrorCode::InvalidOwner.into());
    }
    // TODO: Check GlobalProfile address
    if profile.user != user {
      return Err(ErrorCode::InvalidUser.into());
    }

    let profile = &mut ctx.accounts.global_profile;

    profile.is_blacklisted = is_blacklisted;

    Ok(())
  }

  pub fn create_global_profile(
    ctx: Context<CreateGlobalProfileContext>,
    _nonce: u8,
    user: Pubkey,
  ) -> ProgramResult {
    msg!("Coin98Starship: Instruction_CreateGlobalProfile");

    let profile = &mut ctx.accounts.global_profile;

    profile.user = user;

    Ok(())
  }

  pub fn create_local_profile(
    ctx: Context<CreateLocalProfileContext>,
    _nonce: u8,
    user: Pubkey,
  ) -> ProgramResult {
    msg!("Coin98Starship: Instruction_CreateLocalProfile");

    let launchpad = &ctx.accounts.launchpad;

    let profile = &mut ctx.accounts.local_profile;

    profile.launchpad = *launchpad.key;
    profile.user = user;

    Ok(())
  }
}

#[derive(Accounts)]
#[instruction(launchpad_path: Vec<u8>, launchpad_nonce: u8)]
pub struct CreateLaunchpadContext<'info> {

  #[account(signer)]
  pub root: AccountInfo<'info>,

  #[account(init, seeds = [
    &[8, 201, 24, 140, 93, 100, 30, 148],
    &*launchpad_path,
  ], bump = launchpad_nonce, payer = root, space = 375)]
  pub launchpad: Account<'info, Launchpad>,

  pub rent: Sysvar<'info, Rent>,

  pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetLaunchpadContext<'info> {

  #[account(signer)]
  pub root: AccountInfo<'info>,

  #[account(mut)]
  pub launchpad: Account<'info, Launchpad>,

  pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct SetLaunchpadStatusContext<'info> {

  #[account(signer)]
  pub root: AccountInfo<'info>,

  #[account(mut)]
  pub launchpad: Account<'info, Launchpad>,
}

#[derive(Accounts)]
pub struct RegisterContext<'info> {

  pub launchpad: Account<'info, Launchpad>,

  #[account(signer)]
  pub user: AccountInfo<'info>,

  pub global_profile: Account<'info, GlobalProfile>,

  #[account(mut)]
  pub local_profile: Account<'info, LocalProfile>,

  pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct RedeemBySolContext<'info> {

  pub launchpad: Account<'info, Launchpad>,

  #[account(seeds = [
    &[2, 151, 229, 53, 244,  77, 229,  7],
    launchpad.to_account_info().key.as_ref(),
  ], bump = launchpad.nonce)]
  pub launchpad_signer: AccountInfo<'info>,

  #[account(signer)]
  pub user: AccountInfo<'info>,

  pub global_profile: Account<'info, GlobalProfile>,

  #[account(mut)]
  pub local_profile: Account<'info, LocalProfile>,

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

  pub launchpad: Account<'info, Launchpad>,

  #[account(seeds = [
    &[2, 151, 229, 53, 244,  77, 229,  7],
    launchpad.to_account_info().key.as_ref(),
  ], bump = launchpad.nonce)]
  pub launchpad_signer: AccountInfo<'info>,

  #[account(signer)]
  pub user: AccountInfo<'info>,

  pub global_profile: Account<'info, GlobalProfile>,

  #[account(mut)]
  pub local_profile: Account<'info, LocalProfile>,

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
pub struct SetBlacklistContext<'info> {

  #[account(signer)]
  pub root: AccountInfo<'info>,

  #[account(mut)]
  pub global_profile: Account<'info, GlobalProfile>,
}

#[derive(Accounts)]
#[instruction(profile_nonce: u8, user: Pubkey)]
pub struct CreateGlobalProfileContext<'info> {

  #[account(signer)]
  pub payer: AccountInfo<'info>,

  #[account(init, seeds = [
    &[139, 126, 195, 157, 204, 134, 142, 146],
    &[32, 40, 118, 173, 164, 46, 192, 86],
    user.as_ref(),
  ], bump = profile_nonce, payer = payer, space = 49)]
  pub global_profile: Account<'info, GlobalProfile>,

  pub rent: Sysvar<'info, Rent>,

  pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(profile_nonce: u8, user: Pubkey)]
pub struct CreateLocalProfileContext<'info> {

  #[account(signer)]
  pub payer: AccountInfo<'info>,

  pub launchpad: AccountInfo<'info>,

  #[account(init, seeds = [
    &[133, 177, 201, 78, 13, 152, 198, 180],
    launchpad.key.as_ref(),
    user.as_ref(),
  ], bump = profile_nonce, payer = payer, space = 90)]
  pub local_profile: Account<'info, LocalProfile>,

  pub rent: Sysvar<'info, Rent>,

  pub system_program: AccountInfo<'info>,
}

#[account]
pub struct Launchpad {
  pub nonce: u8,
  pub price_in_sol: u64,
  pub price_in_token: u64,
  pub token_program_id: Pubkey,
  pub token0_mint: Pubkey,
  pub token1_mint: Pubkey,
  pub vault_program_id: Pubkey,
  pub vault: Pubkey,
  pub vault_signer: Pubkey,
  pub vault_token0: Pubkey,
  pub vault_token1: Pubkey,
  pub is_private_sale: bool,
  pub private_sale_signature: Vec<u8>,
  pub min_per_tx: u64,
  pub max_per_user: u64,
  pub register_start_timestamp: i64,
  pub register_end_timestamp: i64,
  pub redeem_start_timestamp: i64,
  pub redeem_end_timestamp: i64,
  pub is_active: bool,
}

#[account]
pub struct GlobalProfile {
  pub user: Pubkey,
  pub is_blacklisted: bool,
}

#[account]
pub struct LocalProfile {
  pub launchpad: Pubkey,
  pub user: Pubkey,
  pub is_registered: bool,
  pub redeemed_token: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default)]
pub struct TransferTokenParams {
  pub amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default)]
pub struct WhitelistParams {
  pub index: u32,
  pub address: Pubkey,
}

#[error]
pub enum ErrorCode {

  #[msg("Coin98Starship: Forbidden.")]
  Blacklisted,

  #[msg("Coin98Starship: Time must be set in the future.")]
  FutureTimeRequired,

  #[msg("Coin98Starship: Invalid launchpad.")]
  InvalidLanchpad,

  #[msg("Coin98Starship: Not an owner.")]
  InvalidOwner,

  #[msg("Coin98Starship: Invalid registration time range.")]
  InvalidRegistrationTime,

  #[msg("Coin98Starship: Invalid sale time range.")]
  InvalidSaleTime,

  #[msg("Coin98Starship: Invalid user.")]
  InvalidUser,

  #[msg("Coin98Starship: Invalid Vault Program Id.")]
  InvalidVaultProgramId,

  #[msg("Coin98Starship: Invalid Vault.")]
  InvalidVault,

  #[msg("Coin98Starship: Invalid Vault Signer.")]
  InvalidVaultSigner,

  #[msg("Coin98Starship: Invalid Vault Token 0 Account.")]
  InvalidVaultToken0,

  #[msg("Coin98Starship: Invalid Vault Token 1 Account.")]
  InvalidVaultToken1,

  #[msg("Coin98Starship: Min amount reached")]
  MaxAmountReached,

  #[msg("Coin98Starship: Min amount not satisfied.")]
  MinAmountNotSatisfied,

  #[msg("Coin98Starship: Only allowed during sale time.")]
  NotInSaleTime,

  #[msg("Coin98Starship: Not registered.")]
  NotRegistered,

  #[msg("Coin98Starship: Not allowed.")]
  NotWhitelisted,

  #[msg("Coin98Starship: Registration and sale time overlap.")]
  RegistrationAndSaleTimeOverlap,

  #[msg("Coin98Starship: Redeem by SOL not allowed.")]
  RedeemBySolNotAllowed,

  #[msg("Coin98Starship: Redeem by token not allowed.")]
  RedeemByTokenNotAllowed,

  #[msg("Coin98Starship: Transaction failed.")]
  TransactionFailed,
}

pub fn verify_owner(owner: &Pubkey) -> bool {
  let owner_key = owner.to_string();
  let result = constants::ROOT_KEYS.iter().position(|&key| key == &owner_key[..]);
  result != None
}

/// Returns true if a `leaf` can be proved to be a part of a Merkle tree
/// defined by `root`. For this, a `proof` must be provided, containing
/// sibling hashes on the branch from the leaf to the root of the tree. Each
/// pair of leaves and each pair of pre-images are assumed to be sorted.
pub fn verify_proof(proofs: Vec<[u8; 32]>, root: [u8; 32], leaf: [u8; 32]) -> bool {
  let mut computed_hash = leaf;
  for proof in proofs.into_iter() {
    if computed_hash < proof {
      // Hash(current computed hash + current element of the proof)
      computed_hash = hashv(&[&computed_hash, &proof]).to_bytes();
    } else {
      // Hash(current element of the proof + current computed hash)
      computed_hash = hashv(&[&proof, &computed_hash]).to_bytes();
    }
  }
  // Check if the computed hash (root) is equal to the provided root
  computed_hash == root
}
