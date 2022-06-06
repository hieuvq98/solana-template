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
) -> std::result::Result<(), ProgramError> {
  let withdraw_params = WithdrawTokenParams {
    amount: *amount,
  };
  let mut withdraw_data: Vec<u8> = Vec::new();
  withdraw_data.extend_from_slice(&[136, 235, 181, 5, 101, 109, 57, 81]);
  withdraw_data.extend_from_slice(&withdraw_params.try_to_vec().unwrap());

  let instruction = solana_program::instruction::Instruction {
    program_id: *vault_program.key,
    accounts: vec![
      solana_program::instruction::AccountMeta::new(*authority.key, true),
      solana_program::instruction::AccountMeta::new_readonly(*vault.key, false),
      solana_program::instruction::AccountMeta::new_readonly(*vault_signer.key, false),
      solana_program::instruction::AccountMeta::new(*vault_token.key, false),
      solana_program::instruction::AccountMeta::new(*user_token.key, false),
      solana_program::instruction::AccountMeta::new_readonly(spl_token::ID, false),
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
