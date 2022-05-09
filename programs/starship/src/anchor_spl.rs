use anchor_lang::{
  AnchorDeserialize,
  AnchorSerialize,
  prelude::{
    AccountInfo,
    borsh,
  },
};
use solana_program::{
  program_error::{
    ProgramError,
  },
};
use crate::{
  spl_token,
};

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
    program_id: spl_token::ID,
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
