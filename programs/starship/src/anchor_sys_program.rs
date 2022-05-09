use anchor_lang::{
  prelude::{
    AccountInfo,
  },
};
use solana_program::{
  program_error::{
    ProgramError,
  },
};

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
