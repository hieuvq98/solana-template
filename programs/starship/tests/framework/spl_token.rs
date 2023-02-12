use std::{
  mem::{
    size_of,
  },
};
use arrayref::{
  array_mut_ref,
  array_ref,
  array_refs,
  mut_array_refs,
};
use num_enum::{
  TryFromPrimitive,
};
use solana_sdk::{
  instruction::{
    AccountMeta,
    Instruction,
  },
  program_error::{
    ProgramError,
  },
  program_option::{
    COption,
  },
  program_pack::{
    IsInitialized,
    Pack,
    Sealed,
  },
  pubkey::{
    Pubkey,
  },
  sysvar::{
    rent::{
      ID as RENT_SYSVAR_ID,
    }
  }
};

// TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
pub const ID: Pubkey = Pubkey::new_from_array([6,221,246,225,215,101,161,147,217,203,225,70,206,235,121,172,28,180,133,237,95,91,55,145,58,140,245,133,126,255,0,169]);

pub const TOKEN_ACCOUNT_LENGTH: usize = 165;
pub const TOKEN_MINT_LENGTH: usize = 82;

/// Account data.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TokenAccount {
  /// The mint associated with this account
  pub mint: Pubkey,
  /// The owner of this account.
  pub owner: Pubkey,
  /// The amount of tokens this account holds.
  pub amount: u64,
  /// If `delegate` is `Some` then `delegated_amount` represents
  /// the amount authorized by the delegate
  pub delegate: COption<Pubkey>,
  /// The account's state
  pub state: AccountState,
  /// If is_native.is_some, this is a native token, and the value logs the rent-exempt reserve. An
  /// Account is required to be rent-exempt, so the value is used by the Processor to ensure that
  /// wrapped SOL accounts do not drop below this threshold.
  pub is_native: COption<u64>,
  /// The amount delegated
  pub delegated_amount: u64,
  /// Optional authority to close the account.
  pub close_authority: COption<Pubkey>,
}

impl TokenAccount {
  /// Checks if account is frozen
  pub fn is_frozen(&self) -> bool {
    self.state == AccountState::Frozen
  }
  /// Checks if account is native
  pub fn is_native(&self) -> bool {
    self.is_native.is_some()
  }
}
impl Sealed for TokenAccount {}
impl IsInitialized for TokenAccount {
  fn is_initialized(&self) -> bool {
    self.state != AccountState::Uninitialized
  }
}
impl Pack for TokenAccount {
  const LEN: usize = 165;
  fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
    let src = array_ref![src, 0, 165];
    let (mint, owner, amount, delegate, state, is_native, delegated_amount, close_authority) =
      array_refs![src, 32, 32, 8, 36, 1, 12, 8, 36];
    Ok(TokenAccount {
      mint: Pubkey::new_from_array(*mint),
      owner: Pubkey::new_from_array(*owner),
      amount: u64::from_le_bytes(*amount),
      delegate: unpack_coption_key(delegate)?,
      state: AccountState::try_from_primitive(state[0])
        .or(Err(ProgramError::InvalidAccountData))?,
      is_native: unpack_coption_u64(is_native)?,
      delegated_amount: u64::from_le_bytes(*delegated_amount),
      close_authority: unpack_coption_key(close_authority)?,
    })
  }
  fn pack_into_slice(&self, dst: &mut [u8]) {
    let dst = array_mut_ref![dst, 0, 165];
    let (
      mint_dst,
      owner_dst,
      amount_dst,
      delegate_dst,
      state_dst,
      is_native_dst,
      delegated_amount_dst,
      close_authority_dst,
    ) = mut_array_refs![dst, 32, 32, 8, 36, 1, 12, 8, 36];
    let &TokenAccount {
      ref mint,
      ref owner,
      amount,
      ref delegate,
      state,
      ref is_native,
      delegated_amount,
      ref close_authority,
    } = self;
    mint_dst.copy_from_slice(mint.as_ref());
    owner_dst.copy_from_slice(owner.as_ref());
    *amount_dst = amount.to_le_bytes();
    pack_coption_key(delegate, delegate_dst);
    state_dst[0] = state as u8;
    pack_coption_u64(is_native, is_native_dst);
    *delegated_amount_dst = delegated_amount.to_le_bytes();
    pack_coption_key(close_authority, close_authority_dst);
  }
}

/// Account state.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, TryFromPrimitive)]
pub enum AccountState {
  /// Account is not yet initialized
  Uninitialized,
  /// Account is initialized; the account owner and/or delegate may perform permitted operations
  /// on this account
  Initialized,
  /// Account has been frozen by the mint freeze authority. Neither the account owner nor
  /// the delegate are able to perform operations on this account.
  Frozen,
}

impl Default for AccountState {
  fn default() -> Self {
    AccountState::Uninitialized
  }
}

/// Mint data.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TokenMint {
  /// Optional authority used to mint new tokens. The mint authority may only be provided during
  /// mint creation. If no mint authority is present then the mint has a fixed supply and no
  /// further tokens may be minted.
  pub mint_authority: COption<Pubkey>,
  /// Total supply of tokens.
  pub supply: u64,
  /// Number of base 10 digits to the right of the decimal place.
  pub decimals: u8,
  /// Is `true` if this structure has been initialized
  pub is_initialized: bool,
  /// Optional authority to freeze token accounts.
  pub freeze_authority: COption<Pubkey>,
}
impl Sealed for TokenMint {}
impl IsInitialized for TokenMint {
  fn is_initialized(&self) -> bool {
    self.is_initialized
  }
}
impl Pack for TokenMint {
  const LEN: usize = 82;
  fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
    let src = array_ref![src, 0, 82];
    let (mint_authority, supply, decimals, is_initialized, freeze_authority) =
      array_refs![src, 36, 8, 1, 1, 36];
    let mint_authority = unpack_coption_key(mint_authority)?;
    let supply = u64::from_le_bytes(*supply);
    let decimals = decimals[0];
    let is_initialized = match is_initialized {
      [0] => false,
      [1] => true,
      _ => return Err(ProgramError::InvalidAccountData),
    };
    let freeze_authority = unpack_coption_key(freeze_authority)?;
    Ok(TokenMint {
      mint_authority,
      supply,
      decimals,
      is_initialized,
      freeze_authority,
    })
  }
  fn pack_into_slice(&self, dst: &mut [u8]) {
    let dst = array_mut_ref![dst, 0, 82];
    let (
      mint_authority_dst,
      supply_dst,
      decimals_dst,
      is_initialized_dst,
      freeze_authority_dst,
    ) = mut_array_refs![dst, 36, 8, 1, 1, 36];
    let &TokenMint {
      ref mint_authority,
      supply,
      decimals,
      is_initialized,
      ref freeze_authority,
    } = self;
    pack_coption_key(mint_authority, mint_authority_dst);
    *supply_dst = supply.to_le_bytes();
    decimals_dst[0] = decimals;
    is_initialized_dst[0] = is_initialized as u8;
    pack_coption_key(freeze_authority, freeze_authority_dst);
  }
}

#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
pub enum AuthorityType {
  MintTokens,
  FreezeAccount,
  AccountOwner,
  CloseAccount,
}

impl AuthorityType {
  fn into(&self) -> u8 {
    match self {
      AuthorityType::MintTokens => 0,
      AuthorityType::FreezeAccount => 1,
      AuthorityType::AccountOwner => 2,
      AuthorityType::CloseAccount => 3,
    }
  }
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum TokenInstruction {
  InitializeMint {
    decimals: u8,
    mint_authority: Pubkey,
    freeze_authority: COption<Pubkey>,
  },
  InitializeAccount,
  InitializeMultisig {
    m: u8,
  },
  Transfer {
    amount: u64,
  },
  Approve {
    amount: u64,
  },
  Revoke,
  SetAuthority {
    authority_type: AuthorityType,
    new_authority: COption<Pubkey>,
  },
  MintTo {
    amount: u64,
  },
  Burn {
    amount: u64,
  },
  CloseAccount,
  FreezeAccount,
  ThawAccount,
}

impl TokenInstruction {
  pub fn pack(&self) -> Vec<u8> {
    let mut buf = Vec::with_capacity(size_of::<Self>());
    match self {
      &Self::InitializeMint {
        ref mint_authority,
        ref freeze_authority,
        decimals,
      } => {
        buf.push(0);
        buf.push(decimals);
        buf.extend_from_slice(mint_authority.as_ref());
        pack_pubkey_option(freeze_authority, &mut buf);
      }
      Self::InitializeAccount => buf.push(1),
      &Self::InitializeMultisig { m } => {
          buf.push(2);
          buf.push(m);
      }
      &Self::Transfer { amount } => {
          buf.push(3);
          buf.extend_from_slice(&amount.to_le_bytes());
      }
      &Self::Approve { amount } => {
          buf.push(4);
          buf.extend_from_slice(&amount.to_le_bytes());
      }
      &Self::MintTo { amount } => {
          buf.push(7);
          buf.extend_from_slice(&amount.to_le_bytes());
      }
      &Self::Burn { amount } => {
          buf.push(8);
          buf.extend_from_slice(&amount.to_le_bytes());
      }
      Self::Revoke => buf.push(5),
      Self::SetAuthority {
          authority_type,
          ref new_authority,
      } => {
          buf.push(6);
          buf.push(authority_type.into());
          pack_pubkey_option(new_authority, &mut buf);
      }
      Self::CloseAccount => buf.push(9),
      Self::FreezeAccount => buf.push(10),
      Self::ThawAccount => buf.push(11),
    };
    buf
  }

}

pub fn initialize_mint_instruction(
  mint_address: &Pubkey,
  decimals: u8,
  mint_authority_address: &Pubkey,
  freeze_authority_address: Option<&Pubkey>,
) -> Instruction {

  let data = TokenInstruction::InitializeMint {
    decimals,
    mint_authority: *mint_authority_address,
    freeze_authority: match freeze_authority_address {
      Some(ref pubkey) => COption::Some(**pubkey),
      None => COption::None,
    },
  }.pack();

  let accounts = vec![
    AccountMeta::new(*mint_address, false),
    AccountMeta::new_readonly(RENT_SYSVAR_ID, false),
  ];

  Instruction {
    accounts,
    data,
    program_id: ID,
  }
}

pub fn mint_token_instruction(
  authority_address: &Pubkey,
  mint_address: &Pubkey,
  destination_address: &Pubkey,
  amount: u64,
) -> Instruction {

  let data = TokenInstruction::MintTo {
    amount
  }.pack();

  let accounts = vec![
    AccountMeta::new(*mint_address, false),
    AccountMeta::new(*destination_address, false),
    AccountMeta::new_readonly(*authority_address, true),
  ];

  Instruction {
    accounts,
    data,
    program_id: ID,
  }
}

pub fn transfer_token_instruction(
  owner_address: &Pubkey,
  source_address: &Pubkey,
  destination_address: &Pubkey,
  amount: u64,
) -> Instruction {

  let data = TokenInstruction::Transfer {
    amount
  }.pack();

  let accounts = vec![
    AccountMeta::new(*source_address, false),
    AccountMeta::new(*destination_address, false),
    AccountMeta::new_readonly(*owner_address, true),
  ];

  Instruction {
    accounts,
    data,
    program_id: ID,
  }
}

// HELPERS
fn pack_pubkey_option(value: &COption<Pubkey>, buf: &mut Vec<u8>) {
  match *value {
    COption::Some(ref key) => {
      buf.push(1);
      buf.extend_from_slice(&key.to_bytes());
    }
    COption::None => buf.push(0),
  }
}

fn pack_coption_key(src: &COption<Pubkey>, dst: &mut [u8; 36]) {
  let (tag, body) = mut_array_refs![dst, 4, 32];
  match src {
    COption::Some(key) => {
      *tag = [1, 0, 0, 0];
      body.copy_from_slice(key.as_ref());
    }
    COption::None => {
      *tag = [0; 4];
    }
  }
}

fn unpack_coption_key(src: &[u8; 36]) -> Result<COption<Pubkey>, ProgramError> {
  let (tag, body) = array_refs![src, 4, 32];
  match *tag {
    [0, 0, 0, 0] => Ok(COption::None),
    [1, 0, 0, 0] => Ok(COption::Some(Pubkey::new_from_array(*body))),
    _ => Err(ProgramError::InvalidAccountData),
  }
}

fn pack_coption_u64(src: &COption<u64>, dst: &mut [u8; 12]) {
  let (tag, body) = mut_array_refs![dst, 4, 8];
  match src {
    COption::Some(amount) => {
      *tag = [1, 0, 0, 0];
      *body = amount.to_le_bytes();
    }
    COption::None => {
      *tag = [0; 4];
    }
  }
}
fn unpack_coption_u64(src: &[u8; 12]) -> Result<COption<u64>, ProgramError> {
  let (tag, body) = array_refs![src, 4, 8];
  match *tag {
    [0, 0, 0, 0] => Ok(COption::None),
    [1, 0, 0, 0] => Ok(COption::Some(u64::from_le_bytes(*body))),
    _ => Err(ProgramError::InvalidAccountData),
  }
}
