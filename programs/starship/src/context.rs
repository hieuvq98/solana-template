use anchor_lang::prelude::*;

use crate::constant::{
  GLOBAL_PROFILE_SEED_1,
  GLOBAL_PROFILE_SEED_2,
  LAUNCHPAD_SEED_1,
  LOCAL_PROFILE_SEED_1,
  SIGNER_SEED_1,
};
use crate::error::ErrorCode;
use crate::state::{
  GlobalProfile,
  Launchpad,
  LocalProfile,
};
use crate::external::spl_token::is_token_program;

#[derive(Accounts)]
#[instruction(launchpad_path: Vec<u8>, _launchpad_nonce: u8)]
pub struct CreateLaunchpadContext<'info> {

  /// CHECK: program owner, verified using #access_control
  #[account(mut)]
  pub root: Signer<'info>,

  #[account(
    init,
    seeds = [
      &LAUNCHPAD_SEED_1,
      &*launchpad_path,
    ],
    bump,
    payer = root,
    space = 16 + Launchpad::LEN,
  )]
  pub launchpad: Account<'info, Launchpad>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetLaunchpadContext<'info> {

  /// CHECK: program owner, verified using #access_control
  pub root: Signer<'info>,

  #[account(mut)]
  pub launchpad: Account<'info, Launchpad>,
}

#[derive(Accounts)]
pub struct SetLaunchpadStatusContext<'info> {

  /// CHECK: program owner, verified using #access_control
  pub root: Signer<'info>,

  #[account(mut)]
  pub launchpad: Account<'info, Launchpad>,
}

#[derive(Accounts)]
pub struct RegisterContext<'info> {

  pub launchpad: Account<'info, Launchpad>,

  /// CHECK: public user
  pub user: Signer<'info>,

  #[account(
    seeds = [
      &GLOBAL_PROFILE_SEED_1,
      &GLOBAL_PROFILE_SEED_2,
      user.key().as_ref(),
    ],
    bump = global_profile.nonce,
    constraint = global_profile.user == user.key() @ErrorCode::InvalidAccount,
  )]
  pub global_profile: Account<'info, GlobalProfile>,

  #[account(
    mut,
    seeds = [
      &LOCAL_PROFILE_SEED_1,
      launchpad.key().as_ref(),
      user.key().as_ref(),
    ],
    bump = local_profile.nonce,
    constraint = local_profile.launchpad == launchpad.key() @ErrorCode::InvalidAccount,
    constraint = local_profile.user == user.key() @ErrorCode::InvalidAccount,
  )]
  pub local_profile: Account<'info, LocalProfile>,
}

#[derive(Accounts)]
pub struct RedeemBySolContext<'info> {

  pub launchpad: Account<'info, Launchpad>,

  /// CHECK: PDA to authorize launchpad tx
  #[account(
    seeds = [
      &SIGNER_SEED_1,
      launchpad.key().as_ref(),
    ],
    bump = launchpad.signer_nonce,
  )]
  pub launchpad_signer: AccountInfo<'info>,

  /// CHECK: public user
  #[account(signer)]
  pub user: AccountInfo<'info>,

  #[account(
    seeds = [
      &GLOBAL_PROFILE_SEED_1,
      &GLOBAL_PROFILE_SEED_2,
      user.key().as_ref(),
    ],
    bump = global_profile.nonce,
    constraint = global_profile.user == user.key() @ErrorCode::InvalidAccount,
  )]
  pub global_profile: Box<Account<'info, GlobalProfile>>,

  #[account(
    mut,
    seeds = [
      &LOCAL_PROFILE_SEED_1,
      launchpad.key().as_ref(),
      user.key().as_ref(),
    ],
    bump = local_profile.nonce,
    constraint = local_profile.launchpad == launchpad.key() @ErrorCode::InvalidAccount,
    constraint = local_profile.user == user.key() @ErrorCode::InvalidAccount,
  )]
  pub local_profile: Box<Account<'info, LocalProfile>>,

  /// CHECK: User token account to receive token sale
  #[account(mut)]
  pub user_token1: AccountInfo<'info>,

  /// CHECK: Vault holding token for sale
  #[account(
    constraint = vault.key() == launchpad.vault @ErrorCode::InvalidAccount,
  )]
  pub vault: AccountInfo<'info>,

  /// CHECK: PDA to hold vault asset
  #[account(
    mut,
    constraint = vault_signer.key() == launchpad.vault_signer @ErrorCode::InvalidAccount,
  )]
  pub vault_signer: AccountInfo<'info>,

  /// CHECK: Vault token account to send token sale
  #[account(
    mut,
    constraint = vault_token1.key() == launchpad.vault_token1 @ErrorCode::InvalidAccount,
  )]
  pub vault_token1: AccountInfo<'info>,

  /// CHECK: Vault holding token for sale
  #[account(
    constraint = vault_program.key() == launchpad.vault_program_id @ErrorCode::InvalidAccount,
  )]
  pub vault_program: AccountInfo<'info>,

  pub system_program: Program<'info, System>,

  /// CHECK: Solana native Token Program
  #[account(
    constraint = is_token_program(&token_program) @ErrorCode::InvalidAccount,
  )]
  pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RedeemByTokenContext<'info> {

  pub launchpad: Account<'info, Launchpad>,

  /// CHECK: PDA to authorize launchpad tx
  #[account(
    seeds = [
      &SIGNER_SEED_1,
      launchpad.key().as_ref(),
    ],
    bump = launchpad.signer_nonce,
  )]
  pub launchpad_signer: AccountInfo<'info>,

  /// CHECK: public user
  pub user: Signer<'info>,

  #[account(
    seeds = [
      &GLOBAL_PROFILE_SEED_1,
      &GLOBAL_PROFILE_SEED_2,
      user.key().as_ref(),
    ],
    bump = global_profile.nonce,
    constraint = global_profile.user == user.key() @ErrorCode::InvalidAccount,
  )]
  pub global_profile: Box<Account<'info, GlobalProfile>>,

  #[account(
    mut,
    seeds = [
      &LOCAL_PROFILE_SEED_1,
      launchpad.key().as_ref(),
      user.key().as_ref(),
    ],
    bump = local_profile.nonce,
    constraint = local_profile.launchpad == launchpad.key() @ErrorCode::InvalidAccount,
    constraint = local_profile.user == user.key() @ErrorCode::InvalidAccount,
  )]
  pub local_profile: Box<Account<'info, LocalProfile>>,

  /// CHECK: User token account to buy token
  #[account(mut)]
  pub user_token0: AccountInfo<'info>,

  /// CHECK: User token account to receive token sale
  #[account(mut)]
  pub user_token1: AccountInfo<'info>,

  /// CHECK: Vault holding token for sale
  #[account(
    constraint = vault.key() == launchpad.vault @ErrorCode::InvalidAccount,
  )]
  pub vault: AccountInfo<'info>,

  /// CHECK: PDA to hold vault asset
  #[account(
    mut,
    constraint = vault_signer.key() == launchpad.vault_signer @ErrorCode::InvalidAccount,
  )]
  pub vault_signer: AccountInfo<'info>,

  /// CHECK: Vault token account to receive token
  #[account(
    mut,
    constraint = vault_token0.key() == launchpad.vault_token0 @ErrorCode::InvalidAccount,
  )]
  pub vault_token0: AccountInfo<'info>,

  /// CHECK: Vault token account to send token sale
  #[account(
    mut,
    constraint = vault_token1.key() == launchpad.vault_token1 @ErrorCode::InvalidAccount,
  )]
  pub vault_token1: AccountInfo<'info>,

  /// CHECK: Vault holding token for sale
  #[account(
    constraint = vault_program.key() == launchpad.vault_program_id @ErrorCode::InvalidAccount,
  )]
  pub vault_program: AccountInfo<'info>,

  /// CHECK: Solana native Token Program
  #[account(
    constraint = is_token_program(&token_program) @ErrorCode::InvalidAccount,
  )]
  pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct SetBlacklistContext<'info> {

  /// CHECK: program owner, verified using #access_control
  #[account(signer)]
  pub root: AccountInfo<'info>,

  #[account(
    mut,
    seeds = [
      &GLOBAL_PROFILE_SEED_1,
      &GLOBAL_PROFILE_SEED_2,
      user.as_ref(),
    ],
    bump = global_profile.nonce,
    constraint = global_profile.user == user @ErrorCode::InvalidAccount,
  )]
  pub global_profile: Account<'info, GlobalProfile>,
}

#[derive(Accounts)]
#[instruction(_profile_nonce: u8, user: Pubkey)]
pub struct CreateGlobalProfileContext<'info> {

  /// CHECK: Fee payer
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
    init,
    seeds = [
      &GLOBAL_PROFILE_SEED_1,
      &GLOBAL_PROFILE_SEED_2,
      user.as_ref(),
    ],
    bump,
    payer = payer,
    space = 16 + GlobalProfile::LEN,
  )]
  pub global_profile: Account<'info, GlobalProfile>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_profile_nonce: u8, user: Pubkey)]
pub struct CreateLocalProfileContext<'info> {

  /// CHECK: Fee payer
  #[account(mut)]
  pub payer: Signer<'info>,

  pub launchpad: Account<'info, Launchpad>,

  #[account(
    init,
    seeds = [
      &LOCAL_PROFILE_SEED_1,
      launchpad.key().as_ref(),
      user.as_ref(),
    ],
    bump,
    payer = payer,
    space = 16 + LocalProfile::LEN,
  )]
  pub local_profile: Account<'info, LocalProfile>,

  pub system_program: Program<'info, System>,
}

