use anchor_lang::prelude::*;
use anchor_lang::solana_program;

#[program]
mod coin98_lunapad {
  use super::*;

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
  ) -> ProgramResult {
    msg!("Coin98Lunapad: Instruction_CreateLaunchpad");

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

  pub fn register(
    ctx: Context<RegisterContext>,
  ) -> ProgramResult {
    msg!("Coin98Lunapad: Instruction_Register");

    let _launchpad = &ctx.accounts.launchpad;
    let _global_profile = &ctx.accounts.global_profile;
    let _local_profile = &ctx.accounts.local_profile;

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
    let _global_profile = &ctx.accounts.global_profile;
    let _local_profile = &ctx.accounts.local_profile;
    let user_token1 = &ctx.accounts.user_token1;
    let vault = &ctx.accounts.vault;
    let vault_signer = &ctx.accounts.vault_signer;
    let vault_token1 = &ctx.accounts.vault_token1;
    let vault_program = &ctx.accounts.vault_program;
    let token_program = &ctx.accounts.token_program;

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
    let _global_profile = &ctx.accounts.global_profile;
    let _local_profile = &ctx.accounts.local_profile;
    let user_token0 = &ctx.accounts.user_token1;
    let user_token1 = &ctx.accounts.user_token1;
    let vault = &ctx.accounts.vault;
    let vault_signer = &ctx.accounts.vault_signer;
    let vault_token0 = &ctx.accounts.vault_token1;
    let vault_token1 = &ctx.accounts.vault_token1;
    let vault_program = &ctx.accounts.vault_program;
    let token_program = &ctx.accounts.token_program;

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
}

#[derive(Accounts)]
#[instruction(launchpad_path: Vec<u8>, launchpad_nonce: u8, _signer_nonce: u8)]
pub struct CreateLaunchpadContext<'info> {

  #[account(signer)]
  pub payer: AccountInfo<'info>,

  #[account(init, seeds = [
    &[8, 201, 24, 140, 93, 100, 30, 148],
    &*launchpad_path,
    &[launchpad_nonce]
  ], payer = payer, space = 340)]
  pub launchpad: ProgramAccount<'info, Launchpad>,

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

#[derive(Accounts)]
pub struct RegisterContext<'info> {

  #[account(signer)]
  pub user: AccountInfo<'info>,

  pub launchpad: ProgramAccount<'info, Launchpad>,

  pub global_profile: ProgramAccount<'info, GlobalProfile>,

  pub local_profile: ProgramAccount<'info, LocalProfile>,
}

#[derive(Accounts)]
pub struct RedeemBySolContext<'info> {

  #[account(signer)]
  pub user: AccountInfo<'info>,

  pub launchpad: ProgramAccount<'info, Launchpad>,

  #[account(seeds = [
    &[2, 151, 229, 53, 244,  77, 229,  7],
    launchpad.to_account_info().key.as_ref(),
    &[launchpad.nonce],
  ])]
  pub launchpad_signer: AccountInfo<'info>,

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

  pub vault_program: AccountInfo<'info>,

  pub system_program: AccountInfo<'info>,

  pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RedeemByTokenContext<'info> {

  #[account(signer)]
  pub user: AccountInfo<'info>,

  pub launchpad: ProgramAccount<'info, Launchpad>,

  #[account(seeds = [
    &[2, 151, 229, 53, 244,  77, 229,  7],
    launchpad.to_account_info().key.as_ref(),
    &[launchpad.nonce],
  ])]
  pub launchpad_signer: AccountInfo<'info>,

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

  pub vault_program: AccountInfo<'info>,

  pub token_program: AccountInfo<'info>,
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

  #[msg("Coin98Lunapad: Not an owner.")]
  InvalidOwner,

  #[msg("Coin98Lunapad: Not new owner.")]
  InvalidNewOwner,

  #[msg("Coin98Lunapad: Transaction failed.")]
  TransactionFailed,
}

