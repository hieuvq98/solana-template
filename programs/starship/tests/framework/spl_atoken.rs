use solana_sdk::{
  instruction::{
    AccountMeta,
    Instruction,
  },
  pubkey::{
    Pubkey,
  },
  system_program::{
    ID as SYSTEM_PROGRAM_ID,
  },
  sysvar::{
    rent::{
      ID as SYSVAR_RENT_ID,
    },
  },
};
use super::{
  spl_token::{
    ID as TOKEN_PROGRAM_ID,
  },
};

// ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL
pub const ID: Pubkey = Pubkey::new_from_array([140,151,37,143,78,36,137,241,187,61,16,41,20,142,13,131,11,90,19,153,218,255,16,132,4,142,123,216,219,233,248,89]);

pub fn create_associated_token_account_instruction(
  payer_address: &Pubkey,
  wallet_address: &Pubkey,
  token_mint_address: &Pubkey,
) -> Instruction {

  let data: Vec<u8> = vec![0u8];

  let (associated_account_address, _) = find_associated_token_account_address(
    wallet_address,
    token_mint_address,
  );
  let accounts = vec![
    AccountMeta::new(*payer_address, true),
    AccountMeta::new(associated_account_address, false),
    AccountMeta::new_readonly(*wallet_address, false),
    AccountMeta::new_readonly(*token_mint_address, false),
    AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
    AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),
    AccountMeta::new_readonly(SYSVAR_RENT_ID, false),
  ];

  Instruction {
    data,
    accounts,
    program_id: ID,
  }
}

pub fn find_associated_token_account_address(
  wallet_address: &Pubkey,
  token_mint_address: &Pubkey,
) -> (Pubkey, u8) {
  Pubkey::find_program_address(
    &[
      &wallet_address.to_bytes(),
      &TOKEN_PROGRAM_ID.to_bytes(),
      &token_mint_address.to_bytes(),
    ],
    &ID,
  )
}

pub fn get_associated_token_account_address(
  wallet_address: &Pubkey,
  token_mint_address: &Pubkey,
) -> Pubkey {
  let (addr, _) = find_associated_token_account_address(wallet_address, token_mint_address);
  addr
}
