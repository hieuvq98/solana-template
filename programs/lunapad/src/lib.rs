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
    vault: Pubkey,
    vault_sol: Pubkey,
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
    launchpad.vault = vault;
    launchpad.vault_sol = vault_sol;
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
    user: Pubkey,
  ) -> ProgramResult {
    msg!("Coin98Lunapad: Instruction_CreateGlobalProfile");

    let profile = &mut ctx.accounts.global_profile;

    profile.user = user;

    Ok(())
  }

  pub fn create_local_profile(
    ctx: Context<CreateLocalProfileContext>,
    _nonce: u8,
    launchpad: Pubkey,
    user: Pubkey,
  ) -> ProgramResult {
    msg!("Coin98Lunapad: Instruction_CreateLocalProfile");

    let profile = &mut ctx.accounts.local_profile;

    profile.launchpad = launchpad;
    profile.user = user;

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
  ], payer = payer, space = 2233)]
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
  ], payer = payer, space = 2233)]
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
  ], payer = payer, space = 2233)]
  pub local_profile: ProgramAccount<'info, LocalProfile>,

  pub rent: Sysvar<'info, Rent>,

  pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct EnrollContext<'info> {

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

  pub global_profile: ProgramAccount<'info, GlobalProfile>,

  #[account(mut)]
  pub local_profile: ProgramAccount<'info, LocalProfile>,

  #[account(mut)]
  pub vault_sol: AccountInfo<'info>,

  #[account(mut)]
  pub vault_token1: AccountInfo<'info>,

  #[account(mut)]
  pub user_token1: AccountInfo<'info>,

  pub system_program: AccountInfo<'info>,

  pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RedeemByTokenContext<'info> {

  #[account(signer)]
  pub user: AccountInfo<'info>,

  pub launchpad: ProgramAccount<'info, Launchpad>,

  pub global_profile: ProgramAccount<'info, GlobalProfile>,

  #[account(mut)]
  pub local_profile: ProgramAccount<'info, LocalProfile>,

  #[account(mut)]
  pub vault_token0: AccountInfo<'info>,

  #[account(mut)]
  pub user_token0: AccountInfo<'info>,

  #[account(mut)]
  pub user_token1: AccountInfo<'info>,

  #[account(mut)]
  pub vault_token1: AccountInfo<'info>,

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
  pub sol_vault: Pubkey,
  pub allow_token: bool,
  pub price_in_token: u64,
  pub vault: Pubkey,
  pub vault_sol: Pubkey,
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
  pub redeemed_token: u64,
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
