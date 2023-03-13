use anchor_lang::prelude::*;
use solana_program::{
  sysvar::{
    instructions::{
      ID as SYSVAR_INSTRUCTION_ID,
    },
  },
};

use crate::{
  constant::{
    FEE_OWNER,
    LAUNCHPAD_PURCHASE_SEED_1,
    LAUNCHPAD_SEED_1,
    SIGNER_SEED_1,
    USER_PROFILE_SEED_1,
    WHITELIST_TOKEN_SEED_1,
  },
  error::{
    ErrorCode,
  },
  state::{
    Launchpad,
    LaunchpadPurchase,
    UserProfile,
    WhitelistToken,
  },
};
use crate::external::{
  anchor_spl_token::{
    TokenAccount,
  },
  spl_token::{
    is_token_program
  },
};

#[derive(Accounts)]
#[instruction(launchpad_path: Vec<u8>)]
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
#[instruction(token_mint: Pubkey)]
pub struct CreateWhitelistTokenContext<'info> {

  /// CHECK: program owner, verified using #access_control
  #[account(mut)]
  pub root: Signer<'info>,

  #[account(
    init,
    seeds = [
      &WHITELIST_TOKEN_SEED_1,
      &token_mint.as_ref()
    ],
    bump,
    payer = root,
    space = 16 + WhitelistToken::LEN
  )]
  pub whitelist: Account<'info, WhitelistToken>,


  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(token_mint: Pubkey)]
pub struct DeleteWhitelistTokenContext<'info> {

  /// CHECK: program owner, verified using #access_control
  #[account(mut)]
  pub root: Signer<'info>,

  #[account(
    mut,
    seeds = [
      &WHITELIST_TOKEN_SEED_1,
      &token_mint.as_ref()
    ],
    bump = whitelist.nonce,
    close = root
  )]
  pub whitelist: Account<'info, WhitelistToken>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetLaunchpadContext<'info> {

  /// CHECK: program owner, verified using #access_control
  pub owner: Signer<'info>,

  #[account(mut)]
  pub launchpad: Account<'info, Launchpad>,
}

#[derive(Accounts)]
pub struct UpdateProtocolFeeContext<'info> {

  /// CHECK: program owner, verified using #access_control
  pub root: Signer<'info>,

  #[account(mut)]
  pub launchpad: Account<'info, Launchpad>,
}

#[derive(Accounts)]
pub struct UpdateSharingFeeContext<'info> {

  /// CHECK: program owner, verified using #access_control
  pub owner: Signer<'info>,

  #[account(mut)]
  pub launchpad: Account<'info, Launchpad>,
}

#[derive(Accounts)]
pub struct TransferLaunchpadOwnershipContext<'info> {

  /// CHECK: program owner, verified using #access_control
  pub owner: Signer<'info>,

  #[account(mut)]
  pub launchpad: Account<'info, Launchpad>,
}

#[derive(Accounts)]
pub struct AcceptLaunchpadOwnershipContext<'info> {

  /// CHECK: program owner, verified using #access_control
  pub new_owner: Signer<'info>,

  #[account(mut)]
  pub launchpad: Account<'info, Launchpad>,
}

#[derive(Accounts)]
#[instruction(token_mint: Pubkey)]
pub struct CreateLaunchpadPurchaseContext<'info> {

  /// CHECK: program owner, verified using #access_control
  #[account(mut)]
  pub owner: Signer<'info>,

  pub launchpad: Account<'info, Launchpad>,

  #[account(
    seeds = [
      &WHITELIST_TOKEN_SEED_1,
      &token_mint.as_ref(),
    ],
    bump = whitelist_token_mint.nonce,
  )]
  pub whitelist_token_mint: Account<'info, WhitelistToken>,

  #[account(
    init,
    seeds = [
      &LAUNCHPAD_PURCHASE_SEED_1,
      launchpad.key().as_ref(),
      token_mint.as_ref(),
    ],
    bump,
    payer = owner,
    space = 16 + LaunchpadPurchase::LEN,
  )]
  pub launchpad_purchase: Account<'info, LaunchpadPurchase>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetLaunchPadPurchaseContext<'info> {

  /// CHECK: program owner, verified using #access_control
  pub owner: Signer<'info>,

  pub launchpad: Box<Account<'info, Launchpad>>,

  /// CHECK: PDA to authorize launchpad tx
  #[account(
    mut,
    seeds = [
      &LAUNCHPAD_PURCHASE_SEED_1,
      launchpad.key().as_ref(),
      launchpad_purchase.token_mint.as_ref()
    ],
    bump = launchpad_purchase.nonce,
  )]
  pub launchpad_purchase: Box<Account<'info, LaunchpadPurchase>>,
}

#[derive(Accounts)]
pub struct SetLaunchpadStatusContext<'info> {

  /// CHECK: program owner, verified using #access_control
  pub owner: Signer<'info>,

  #[account(mut)]
  pub launchpad: Account<'info, Launchpad>,
}

#[derive(Accounts)]
pub struct RegisterContext<'info> {

  pub launchpad: Account<'info, Launchpad>,

  /// CHECK: public user
  pub user: Signer<'info>,

  #[account(
    mut,
    seeds = [
      &USER_PROFILE_SEED_1,
      launchpad.key().as_ref(),
      user.key().as_ref(),
    ],
    bump = user_profile.nonce,
    constraint = user_profile.launchpad == launchpad.key() @ErrorCode::InvalidAccount,
    constraint = user_profile.user == user.key() @ErrorCode::InvalidAccount,
  )]
  pub user_profile: Account<'info, UserProfile>,

  /// CHECK: check by address
  #[account(
    address = SYSVAR_INSTRUCTION_ID,
  )]
  pub sysvar_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RedeemBySolContext<'info> {

  pub launchpad: Account<'info, Launchpad>,

  /// CHECK: PDA to authorize launchpad tx
  #[account(mut,
    seeds = [
      &SIGNER_SEED_1,
      launchpad.key().as_ref(),
    ],
    bump = launchpad.signer_nonce,
  )]
  pub launchpad_signer: AccountInfo<'info>,

  /// CHECK: public user
  #[account(mut, signer)]
  pub user: AccountInfo<'info>,

  #[account(
    mut,
    seeds = [
      &USER_PROFILE_SEED_1,
      launchpad.key().as_ref(),
      user.key().as_ref(),
    ],
    bump = user_profile.nonce,
    constraint = user_profile.launchpad == launchpad.key() @ErrorCode::InvalidAccount,
    constraint = user_profile.user == user.key() @ErrorCode::InvalidAccount,
  )]
  pub user_profile: Box<Account<'info, UserProfile>>,

  /// CHECK: User token account to receive token sale
  #[account(mut)]
  pub user_token_account: AccountInfo<'info>,

  /// CHECK: User token account to receive token sale
  #[account(
    mut,
    constraint = launchpad_token_account.owner == launchpad_signer.key() @ErrorCode::InvalidAccount,
    constraint = launchpad_token_account.mint == launchpad.token_mint @ErrorCode::InvalidAccount,
  )]
  pub launchpad_token_account: Account<'info, TokenAccount>,

  /// CHECK: Fee owner of system fee
  #[account(
    mut,
    constraint = fee_owner.key().to_string() == FEE_OWNER @ErrorCode::InvalidAccount,
  )]
  pub fee_owner: AccountInfo<'info>,

  pub system_program: Program<'info, System>,

  /// CHECK: Solana native Token Program
  #[account(
    constraint = is_token_program(&token_program) @ErrorCode::InvalidAccount,
  )]
  pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RedeemByTokenContext<'info> {

  pub launchpad: Box<Account<'info, Launchpad>>,

  /// CHECK: PDA to authorize launchpad tx
  #[account(
    seeds = [
      &LAUNCHPAD_PURCHASE_SEED_1,
      launchpad.key().as_ref(),
      launchpad_purchase.token_mint.as_ref()
    ],
    bump = launchpad_purchase.nonce,
  )]
  pub launchpad_purchase: Box<Account<'info, LaunchpadPurchase>>,

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
    mut,
    seeds = [
      &USER_PROFILE_SEED_1,
      launchpad.key().as_ref(),
      user.key().as_ref(),
    ],
    bump = user_profile.nonce,
    constraint = user_profile.launchpad == launchpad.key() @ErrorCode::InvalidAccount,
    constraint = user_profile.user == user.key() @ErrorCode::InvalidAccount,
  )]
  pub user_profile: Box<Account<'info, UserProfile>>,

  /// CHECK: User token account to buy token
  #[account(mut)]
  pub user_token0_account: AccountInfo<'info>,

  /// CHECK: User token account to receive token sale
  #[account(mut)]
  pub user_token1_account: AccountInfo<'info>,

  /// CHECK: User token account to receive token sale
  #[account(
    mut,
    constraint = launchpad_token0_account.owner == launchpad_signer.key() @ErrorCode::InvalidAccount,
    constraint = launchpad_token0_account.mint == launchpad_purchase.token_mint @ErrorCode::InvalidAccount,
  )]
  pub launchpad_token0_account: Account<'info, TokenAccount>,

  #[account(
    mut,
    constraint = launchpad_token1_account.owner == launchpad_signer.key() @ErrorCode::InvalidAccount,
    constraint = launchpad_token1_account.mint == launchpad.token_mint @ErrorCode::InvalidAccount,
  )]
  pub launchpad_token1_account: Account<'info, TokenAccount>,

  #[account(
    mut,
    constraint = fee_owner_token0_account.owner.to_string() == FEE_OWNER @ErrorCode::InvalidAccount,
    constraint = fee_owner_token0_account.mint == launchpad_purchase.token_mint @ErrorCode::InvalidAccount,
  )]
  pub fee_owner_token0_account: Account<'info, TokenAccount>,

  /// CHECK: Solana native Token Program
  #[account(
    constraint = is_token_program(&token_program) @ErrorCode::InvalidAccount,
  )]
  pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct CreateUserProfileContext<'info> {

  /// CHECK: Fee payer
  #[account(mut)]
  pub payer: Signer<'info>,

  pub launchpad: Account<'info, Launchpad>,

  #[account(
    init,
    seeds = [
      &USER_PROFILE_SEED_1,
      launchpad.key().as_ref(),
      user.as_ref(),
    ],
    bump,
    payer = payer,
    space = 16 + UserProfile::LEN,
  )]
  pub user_profile: Account<'info, UserProfile>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimPendingTokenContext<'info> {
  /// CHECK: Fee payer
  pub user: Signer<'info>,

  #[account(mut)]
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

  #[account(
    mut,
    seeds = [
      &USER_PROFILE_SEED_1,
      launchpad.key().as_ref(),
      user.key.as_ref(),
    ],
    bump = user_profile.nonce,
  )]
  pub user_profile: Account<'info, UserProfile>,

  #[account(
    mut,
    constraint = launchpad_token1_account.owner == launchpad_signer.key() @ErrorCode::InvalidAccount,
    constraint = launchpad_token1_account.mint == launchpad.token_mint @ErrorCode::InvalidAccount,
  )]
  pub launchpad_token1_account: Account<'info, TokenAccount>,

  /// CHECK: User token account to receive token sale
  #[account(mut)]
  pub user_token1_account: AccountInfo<'info>,

  /// CHECK: Solana native Token Program
  #[account(
    constraint = is_token_program(&token_program) @ErrorCode::InvalidAccount,
  )]
  pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawSolContext<'info> {
  /// CHECK: Root
  #[account(mut)]
  pub owner: Signer<'info>,

  pub launchpad: Account<'info, Launchpad>,

  /// CHECK: PDA to authorize launchpad tx
  #[account(
    mut,
    seeds = [
      &SIGNER_SEED_1,
      launchpad.key().as_ref(),
    ],
    bump = launchpad.signer_nonce,
  )]
  pub launchpad_signer: AccountInfo<'info>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(token_mint: Pubkey, _amount: u64)]
pub struct WithdrawTokenContext<'info> {
  /// CHECK: Root
  #[account(mut)]
  pub owner: Signer<'info>,

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

  #[account(
    mut,
    constraint = from.mint == token_mint @ErrorCode::InvalidAccount,
  )]
  pub from: Account<'info, TokenAccount>,

  /// CHECK: To token account
  #[account(mut)]
  pub to: AccountInfo<'info>,

  /// CHECK: Solana native Token Program
  #[account(
    constraint = is_token_program(&token_program) @ErrorCode::InvalidAccount,
  )]
  pub token_program: AccountInfo<'info>,
}
