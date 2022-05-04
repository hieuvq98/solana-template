use crate::constants::ROOT_KEYS;
use std::convert::{
  TryFrom
};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak::{
  hashv,
};

static TOKEN_PROGRAM_ID: Pubkey = Pubkey::new_from_array([6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172, 28, 180, 133, 237, 95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169]);

pub fn calculate_sub_total(
  amount: u64,
  price_n: u64,
  price_d: u64,
) -> Option<u64> {
  if amount == 0 || price_n == 0 {
    Some(0)
  } else {
    let x = u128::from(amount);
    let n = u128::from(price_n);
    let d = u128::from(price_d);
    let result = x
      .checked_mul(n)?
      .checked_div(d)?;
    let u64_max = u128::from(u64::MAX);
    if result > u64_max {
      None
    } else {
      Some(u64::try_from(result).unwrap())
    }
  }
}

pub fn transfer_lamports<'info>(
  from_pubkey: &AccountInfo<'info>,
  to_pubkey: &AccountInfo<'info>,
  amount: u64,
  signer_seeds: &[&[&[u8]]],
) -> std::result::Result<(), ProgramError> {
  let instruction = &solana_program::system_instruction::transfer(from_pubkey.key, to_pubkey.key, amount);
  if signer_seeds.len() == 0 {
    solana_program::program::invoke(&instruction, &[from_pubkey.clone(), to_pubkey.clone()])
  }
  else {
    solana_program::program::invoke_signed(&instruction, &[from_pubkey.clone(), to_pubkey.clone()], &signer_seeds)
  }
}

pub fn transfer_token<'info>(
  owner: &AccountInfo<'info>,
  from_pubkey: &AccountInfo<'info>,
  to_pubkey: &AccountInfo<'info>,
  amount: u64,
  signer_seeds: &[&[&[u8]]],
) -> std::result::Result<(), ProgramError> {
  let data = TransferTokenParams {
    instruction: 3,
    amount: amount,
  };
  let instruction = solana_program::instruction::Instruction {
    program_id: TOKEN_PROGRAM_ID,
    accounts: vec![
      solana_program::instruction::AccountMeta::new(*from_pubkey.key, false),
      solana_program::instruction::AccountMeta::new(*to_pubkey.key, false),
      solana_program::instruction::AccountMeta::new_readonly(*owner.key, true),
    ],
    data: data.try_to_vec().unwrap(),
  };
  if signer_seeds.len() == 0 {
    solana_program::program::invoke(&instruction, &[from_pubkey.clone(), to_pubkey.clone(), owner.clone()])
  }
  else {
    solana_program::program::invoke_signed(&instruction, &[from_pubkey.clone(), to_pubkey.clone(), owner.clone()], &signer_seeds)
  }
}


#[derive(AnchorSerialize, AnchorDeserialize, Default)]
pub struct TransferTokenParams {
  pub instruction: u8,
  pub amount: u64,
}

pub fn withdraw_token<'info>(
  amount: &u64,
  authority: &AccountInfo<'info>,
  vault: &AccountInfo<'info>,
  vault_signer: &AccountInfo<'info>,
  vault_token: &AccountInfo<'info>,
  user_token: &AccountInfo<'info>,
  vault_program: &AccountInfo<'info>,
  token_program_id: &AccountInfo<'info>,
  signer_seeds: &[&[&[u8]]],
) -> Result<()> {
  let withdraw_params = WithdrawTokenParams {
    amount: *amount,
  };
  let mut withdraw_data: Vec<u8> = Vec::new();
  withdraw_data.extend_from_slice(&[27, 191, 15, 150, 68, 201, 127, 133]);
  withdraw_data.extend_from_slice(&withdraw_params.try_to_vec().unwrap());

  let instruction = solana_program::instruction::Instruction {
    program_id: *vault_program.key,
    accounts: vec![
      solana_program::instruction::AccountMeta::new_readonly(*authority.key, true),
      solana_program::instruction::AccountMeta::new_readonly(*vault.key, false),
      solana_program::instruction::AccountMeta::new_readonly(*vault_signer.key, false),
      solana_program::instruction::AccountMeta::new(*vault_token.key, false),
      solana_program::instruction::AccountMeta::new(*user_token.key, false),
      solana_program::instruction::AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),
    ],
    data: withdraw_data,
  };

  solana_program::program::invoke_signed(&instruction, &[
    authority.clone(),
    vault.clone(),
    vault_signer.clone(),
    vault_token.clone(),
    user_token.clone(),
    token_program_id.clone(),
  ], &signer_seeds)?;

  Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Default)]
pub struct WithdrawTokenParams {
  pub amount: u64,
}

pub fn verify_owner(owner: &Pubkey) -> bool {
  let owner_key = owner.to_string();
  let result = ROOT_KEYS.iter().position(|&key| key == &owner_key[..]);
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

