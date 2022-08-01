use anchor_lang::prelude::*;

use crate::error::{
  ErrorCode,
};
use crate::state::{
  GlobalProfile,
  Launchpad,
  LocalProfile,
};

#[derive(Accounts)]
#[instruction(launchpad_path: Vec<u8>, _launchpad_nonce: u8)]
pub struct CreateLaunchpadContext<'info> {

  /// CHECK: program owner, verified using #access_control
  #[account(mut, signer)]
  pub root: AccountInfo<'info>,

  #[account(
    init,
    seeds = [
      &[8, 201, 24, 140, 93, 100, 30, 148],
      &*launchpad_path,
    ],
    bump,
    payer = root,
    space = Launchpad::LEN,
  )]
  pub launchpad: Account<'info, Launchpad>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetLaunchpadContext<'info> {

  /// CHECK: program owner, verified using #access_control
  #[account(signer)]
  pub root: AccountInfo<'info>,

  #[account(mut)]
  pub launchpad: Account<'info, Launchpad>,
}

#[derive(Accounts)]
pub struct SetLaunchpadStatusContext<'info> {

  /// CHECK: program owner, verified using #access_control
  #[account(signer)]
  pub root: AccountInfo<'info>,

  #[account(mut)]
  pub launchpad: Account<'info, Launchpad>,
}

#[derive(Accounts)]
pub struct RegisterContext<'info> {

  pub launchpad: Account<'info, Launchpad>,

  /// CHECK: public user
  #[account(signer)]
  pub user: AccountInfo<'info>,

  #[account(
    seeds = [
      &[139, 126, 195, 157, 204, 134, 142, 146],
      &[32, 40, 118, 173, 164, 46, 192, 86],
      user.key().as_ref(),
    ],
    bump = global_profile.nonce,
    constraint = global_profile.user == user.key() @ErrorCode::InvalidAccount,
  )]
  pub global_profile: Account<'info, GlobalProfile>,

  #[account(
    mut,
    seeds = [
      &[133, 177, 201, 78, 13, 152, 198, 180],
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
      &[2, 151, 229, 53, 244,  77, 229,  7],
      launchpad.key().as_ref(),
    ],
    bump = launchpad.nonce,
  )]
  pub launchpad_signer: AccountInfo<'info>,

  /// CHECK: public user
  #[account(signer)]
  pub user: AccountInfo<'info>,

  #[account(
    seeds = [
      &[139, 126, 195, 157, 204, 134, 142, 146],
      &[32, 40, 118, 173, 164, 46, 192, 86],
      user.key().as_ref(),
    ],
    bump = global_profile.nonce,
    constraint = global_profile.user == user.key() @ErrorCode::InvalidAccount,
  )]
  pub global_profile: Box<Account<'info, GlobalProfile>>,

  #[account(
    mut,
    seeds = [
      &[133, 177, 201, 78, 13, 152, 198, 180],
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
    constraint = token_program.key() == launchpad.token_program_id @ErrorCode::InvalidAccount,
  )]
  pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RedeemByTokenContext<'info> {

  pub launchpad: Account<'info, Launchpad>,

  /// CHECK: PDA to authorize launchpad tx
  #[account(
    seeds = [
      &[2, 151, 229, 53, 244,  77, 229,  7],
      launchpad.key().as_ref(),
    ],
    bump = launchpad.nonce,
  )]
  pub launchpad_signer: AccountInfo<'info>,

  /// CHECK: public user
  #[account(signer)]
  pub user: AccountInfo<'info>,

  #[account(
    seeds = [
      &[139, 126, 195, 157, 204, 134, 142, 146],
      &[32, 40, 118, 173, 164, 46, 192, 86],
      user.key().as_ref(),
    ],
    bump = global_profile.nonce,
    constraint = global_profile.user == user.key() @ErrorCode::InvalidAccount,
  )]
  pub global_profile: Box<Account<'info, GlobalProfile>>,

  #[account(
    mut,
    seeds = [
      &[133, 177, 201, 78, 13, 152, 198, 180],
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
    constraint = token_program.key() == launchpad.token_program_id @ErrorCode::InvalidAccount,
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
      &[139, 126, 195, 157, 204, 134, 142, 146],
      &[32, 40, 118, 173, 164, 46, 192, 86],
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
  #[account(mut, signer)]
  pub payer: AccountInfo<'info>,

  #[account(
    init,
    seeds = [
      &[139, 126, 195, 157, 204, 134, 142, 146],
      &[32, 40, 118, 173, 164, 46, 192, 86],
      user.as_ref(),
    ],
    bump,
    payer = payer,
    space = GlobalProfile::LEN,
  )]
  pub global_profile: Account<'info, GlobalProfile>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_profile_nonce: u8, user: Pubkey)]
pub struct CreateLocalProfileContext<'info> {

  /// CHECK: Fee payer
  #[account(mut, signer)]
  pub payer: AccountInfo<'info>,

  pub launchpad: Account<'info, Launchpad>,

  #[account(
    init,
    seeds = [
      &[133, 177, 201, 78, 13, 152, 198, 180],
      launchpad.key().as_ref(),
      user.as_ref(),
    ],
    bump,
    payer = payer,
    space = LocalProfile::LEN,
  )]
  pub local_profile: Account<'info, LocalProfile>,

  pub system_program: Program<'info, System>,
}

