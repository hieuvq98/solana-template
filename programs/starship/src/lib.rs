pub mod anchor_spl;
pub mod anchor_sys_program;
pub mod coin98_vault;
pub mod constants;
pub mod shared;
pub mod spl_token;

use anchor_lang::prelude::*;
use anchor_lang::{
  solana_program::{
    keccak::{
      hash,
    },
  },
};
use std::{
  convert::{
    TryInto,
  },
};

declare_id!("SS4VMP9wmqQdehu7Uc6g1Ymsx4BCVVghKp4wRmmy1jj");

#[program]
mod coin98_starship {
  use super::*;

  #[access_control(verify_root(*ctx.accounts.root.key))]
  pub fn create_launchpad(
    ctx: Context<CreateLaunchpadContext>,
    _launchpad_path: Vec<u8>,
    _launchpad_nonce: u8,
    signer_nonce: u8,
  ) -> Result<()> {
    msg!("Coin98Starship: Instruction_CreateLaunchpad");

    let launchpad = &mut ctx.accounts.launchpad;

    launchpad.nonce = signer_nonce;
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
  ) -> Result<()> {
    msg!("Coin98Starship: Instruction_SetLaunchpad");

    let clock = Clock::get().unwrap();

    if register_start_timestamp > register_end_timestamp {
      return Err(error!(ErrorCode::InvalidRegistrationTime));
    }
    if clock.unix_timestamp > register_start_timestamp {
      return Err(error!(ErrorCode::FutureTimeRequired));
    }
    if redeem_start_timestamp > redeem_end_timestamp {
      return Err(error!(ErrorCode::InvalidSaleTime));
    }
    if redeem_start_timestamp < register_end_timestamp {
      return Err(error!(ErrorCode::RegistrationAndSaleTimeOverlap));
    }

    let launchpad = &mut ctx.accounts.launchpad;

    launchpad.price_in_sol_n = price_in_sol_n;
    launchpad.price_in_sol_d = price_in_sol_d;
    launchpad.price_in_token_n = price_in_token_n;
    launchpad.price_in_token_d = price_in_token_d;
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

  #[access_control(verify_root(*ctx.accounts.root.key))]
  pub fn set_launchpad_status(
    ctx: Context<SetLaunchpadStatusContext>,
    is_active: bool,
  ) -> Result<()> {
    msg!("Coin98Starship: Instruction_SetLaunchpadStatus");

    let launchpad = &mut ctx.accounts.launchpad;

    launchpad.is_active = is_active;

    Ok(())
  }

  pub fn register(
    ctx: Context<RegisterContext>,
    index: u32,
    proofs: Vec<[u8; 32]>,
  ) -> Result<()> {
    msg!("Coin98Starship: Instruction_Register");

    let user = &ctx.accounts.user;
    let launchpad = &ctx.accounts.launchpad;
    let global_profile = &ctx.accounts.global_profile;
    let clock = Clock::get().unwrap();

    if global_profile.is_blacklisted {
      return Err(error!(ErrorCode::Blacklisted));
    }
    if clock.unix_timestamp < launchpad.register_start_timestamp || clock.unix_timestamp > launchpad.register_end_timestamp {
      return Err(error!(ErrorCode::NotInRegistrationTime));
    }
    if launchpad.is_private_sale {
      let whitelist = WhitelistParams {
        index: index,
        address: *user.key
      };
      let whitelist_data = whitelist.try_to_vec().unwrap();
      let leaf = hash(&whitelist_data[..]);
      let root: [u8; 32] = launchpad.private_sale_signature.clone().try_into().unwrap();
      if !shared::verify_proof(proofs, root, leaf.to_bytes()) {
        return Err(error!(ErrorCode::NotWhitelisted));
      }
    }

    let local_profile = &mut ctx.accounts.local_profile;
    local_profile.is_registered = true;

    Ok(())
  }

  pub fn redeem_by_sol(
    ctx: Context<RedeemBySolContext>,
    amount: u64,
  ) -> Result<()> {
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
    let clock = Clock::get().unwrap();
    let vault_program = &ctx.accounts.vault_program;
    let token_program = &ctx.accounts.token_program;

    if global_profile.is_blacklisted {
      return Err(error!(ErrorCode::Blacklisted));
    }
    if launchpad.price_in_sol_n == 0u64 {
      return Err(error!(ErrorCode::RedeemBySolNotAllowed));
    }
    if !local_profile.is_registered {
      return Err(error!(ErrorCode::NotRegistered));
    }
    if clock.unix_timestamp < launchpad.redeem_start_timestamp || clock.unix_timestamp > launchpad.redeem_end_timestamp {
      return Err(error!(ErrorCode::NotInSaleTime));
    }
    if launchpad.min_per_tx > 0 && amount < launchpad.min_per_tx {
      return Err(error!(ErrorCode::MinAmountNotSatisfied));
    }
    let redeemed_amount = local_profile.redeemed_token.checked_add(amount).unwrap();
    if launchpad.max_per_user > 0 && redeemed_amount > launchpad.max_per_user {
      return Err(error!(ErrorCode::MaxAmountReached));
    }

    let amount_sol = shared::calculate_sub_total(amount, launchpad.price_in_sol_n, launchpad.price_in_sol_d)
      .unwrap();
    // Transfer lamports
    let result = anchor_sys_program::transfer_lamports(
      &user.to_account_info(),
      &vault_signer.to_account_info(),
      amount_sol,
      &[]
    );
    if result.is_err() {
      return Err(error!(ErrorCode::TransactionFailed));
    }

    let local_profile = &mut ctx.accounts.local_profile;

    local_profile.redeemed_token = redeemed_amount;
    let seeds: &[&[_]] = &[
      &[2, 151, 229, 53, 244,  77, 229,  7],
      launchpad.to_account_info().key.as_ref(),
      &[launchpad.nonce],
    ];

    // Transfer token 1
    let result = coin98_vault::withdraw_token(
      &amount,
      &launchpad_signer.to_account_info(),
      &vault.to_account_info(),
      &vault_signer.to_account_info(),
      &vault_token1.to_account_info(),
      &user_token1.to_account_info(),
      &vault_program.to_account_info(),
      &token_program.to_account_info(),
      &[seeds],
    );

    if result.is_err() {
      return Err(error!(ErrorCode::TransactionFailed));
    }

    Ok(())
  }

  pub fn redeem_by_token(
    ctx: Context<RedeemByTokenContext>,
    amount: u64,
  ) -> Result<()> {
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
    let clock = Clock::get().unwrap();
    let vault_program = &ctx.accounts.vault_program;
    let token_program = &ctx.accounts.token_program;

    if global_profile.is_blacklisted {
      return Err(error!(ErrorCode::Blacklisted));
    }
    if launchpad.price_in_token_n == 0u64 {
      return Err(error!(ErrorCode::RedeemByTokenNotAllowed));
    }
    if clock.unix_timestamp < launchpad.redeem_start_timestamp || clock.unix_timestamp > launchpad.redeem_end_timestamp {
      return Err(error!(ErrorCode::NotInSaleTime));
    }
    if launchpad.min_per_tx > 0 && amount < launchpad.min_per_tx {
      return Err(error!(ErrorCode::MinAmountNotSatisfied));
    }
    let redeemed_amount = local_profile.redeemed_token.checked_add(amount).unwrap();
    if launchpad.max_per_user > 0 && redeemed_amount > launchpad.max_per_user {
      return Err(error!(ErrorCode::MaxAmountReached));
    }

    let amount_token0 = shared::calculate_sub_total(amount, launchpad.price_in_token_n, launchpad.price_in_token_d)
      .unwrap();

    // Transfer token 0
    let result = anchor_spl::transfer_token(
      &user.to_account_info(),
      &user_token0.to_account_info(),
      &vault_token0.to_account_info(),
      amount_token0,
      &[]
    );

    if result.is_err() {
      return Err(error!(ErrorCode::TransactionFailed));
    }

    let local_profile = &mut ctx.accounts.local_profile;

    local_profile.redeemed_token = redeemed_amount;

    let seeds: &[&[_]] = &[
      &[2, 151, 229, 53, 244,  77, 229,  7],
      launchpad.to_account_info().key.as_ref(),
      &[launchpad.nonce],
    ];

    // Transfer token 1
    let result = coin98_vault::withdraw_token(
      &amount,
      &launchpad_signer.to_account_info(),
      &vault.to_account_info(),
      &vault_signer.to_account_info(),
      &vault_token1.to_account_info(),
      &user_token1.to_account_info(),
      &vault_program.to_account_info(),
      &token_program.to_account_info(),
      &[seeds],
    );

    if result.is_err() {
      return Err(error!(ErrorCode::TransactionFailed));
    }

    Ok(())
  }

  #[access_control(verify_root(*ctx.accounts.root.key))]
  pub fn set_blacklist(
    ctx: Context<SetBlacklistContext>,
    _user: Pubkey,
    is_blacklisted: bool,
  ) -> Result<()> {
    msg!("Coin98Starship: Instruction_SetBlacklist");

    let profile = &mut ctx.accounts.global_profile;

    profile.is_blacklisted = is_blacklisted;

    Ok(())
  }

  pub fn create_global_profile(
    ctx: Context<CreateGlobalProfileContext>,
    nonce: u8,
    user: Pubkey,
  ) -> Result<()> {
    msg!("Coin98Starship: Instruction_CreateGlobalProfile");

    let profile = &mut ctx.accounts.global_profile;

    profile.nonce = nonce;
    profile.user = user;

    Ok(())
  }

  pub fn create_local_profile(
    ctx: Context<CreateLocalProfileContext>,
    nonce: u8,
    user: Pubkey,
  ) -> Result<()> {
    msg!("Coin98Starship: Instruction_CreateLocalProfile");

    let launchpad = &ctx.accounts.launchpad;

    let profile = &mut ctx.accounts.local_profile;

    profile.nonce = nonce;
    profile.launchpad = launchpad.key();
    profile.user = user;

    Ok(())
  }
}

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

#[account]
pub struct Launchpad {
  pub nonce: u8,
  pub price_in_sol_n: u64,
  pub price_in_sol_d: u64,
  pub price_in_token_n: u64,
  pub price_in_token_d: u64,
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
  pub owner: Pubkey, // For compability reason
  pub new_owner: Pubkey, // For compability reason
  pub is_active: bool,
}
impl Launchpad {
  pub const LEN: usize = 16 + 1 + 8 + 8 + 8 + 8 + 16 + 16 + 16 + 16 + 16 + 16 + 16 + 16 + 1 + 36 + 8 + 8 + 8 + 8 + 8 + 8 + 32 + 32 + 1;
}

#[account]
pub struct GlobalProfile {
  pub nonce: u8,
  pub user: Pubkey,
  pub is_blacklisted: bool,
}
impl GlobalProfile {
  pub const LEN: usize = 16 + 1 + 32 + 1;
}

#[account]
pub struct LocalProfile {
  pub nonce: u8,
  pub launchpad: Pubkey,
  pub user: Pubkey,
  pub is_registered: bool,
  pub redeemed_token: u64,
}
impl LocalProfile {
  pub const LEN: usize = 16 + 1 + 32 + 32 + 1 + 8;
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

#[error_code]
pub enum ErrorCode {

  #[msg("Coin98Starship: Forbidden.")]
  Blacklisted,

  #[msg("Coin98Starship: Time must be set in the future.")]
  FutureTimeRequired,

  #[msg("Coin98Starship: Invalid account.")]
  InvalidAccount,

  #[msg("Coin98Starship: Not an owner.")]
  InvalidOwner,

  #[msg("Coin98Starship: Invalid registration time range.")]
  InvalidRegistrationTime,

  #[msg("Coin98Starship: Invalid sale time range.")]
  InvalidSaleTime,

  #[msg("Coin98Starship: Max amount reached")]
  MaxAmountReached,

  #[msg("Coin98Starship: Min amount not satisfied.")]
  MinAmountNotSatisfied,

  #[msg("Coin98Starship: Only allowed during registration time.")]
  NotInRegistrationTime,

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

/// Verify proof
pub fn verify_proof(index: u32, user: &Pubkey, proofs: &Vec<[u8; 32]>, launchpad: &Launchpad) -> Result<()> {
  let whitelist_params = WhitelistParams {
    index: index,
    address: *user,
  };
  let whitelist_data = whitelist_params.try_to_vec().unwrap();
  let root: [u8; 32] = launchpad.private_sale_signature.clone().try_into().unwrap();
  let leaf = hash(&whitelist_data[..]);
  if !shared::verify_proof(proofs.to_vec(), root, leaf.to_bytes()) {
    return Err(ErrorCode::NotWhitelisted.into());
  }

  Ok(())
}

pub fn verify_root(user: Pubkey) -> Result<()> {
  let user_key = user.to_string();
  let result = constants::ROOT_KEYS.iter().position(|&key| key == &user_key[..]);
  if result == None {
    return Err(ErrorCode::InvalidOwner.into());
  }

  Ok(())
}
