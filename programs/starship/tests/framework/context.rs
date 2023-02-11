use solana_program_test::{
  BanksClientError,
  ProgramTest,
  ProgramTestContext,
};
use solana_sdk::{
  clock::{
    Clock,
  },
  instruction::{
    Instruction,
    InstructionError,
  },
  pubkey::{
    Pubkey,
  },
  signature::{
    Keypair,
    Signer,
  },
  transaction::{
    Transaction,
    TransactionError,
  },
};

pub fn assert_transaction_result(
  result: Result<(), BanksClientError>,
  is_success: bool,
  expected_error_message: &str,
) {
  assert!(result.is_ok() == is_success);
  if is_success {
    return;
  }
  let error_message = result.unwrap_err().to_string();
  assert!(error_message.contains(&expected_error_message));
}

pub fn assert_instruction_error(
  result: Result<(), BanksClientError>,
  instruction_index: u8,
  error_code: u32,
) {
  assert!(result.is_err());
  let error = result.unwrap_err().unwrap();
  assert_eq!(
    error,
    TransactionError::InstructionError(
      instruction_index,
      InstructionError::Custom(error_code),
    ),
  );
}

pub async fn create_context(
  programs: &[(&str, Pubkey)]
) -> ProgramTestContext {
  let mut context_builder = ProgramTest::default();
  context_builder.prefer_bpf(true);
  for i in 0..programs.len() {
    let program = &programs[i];
    context_builder.add_program(
      program.0,
      program.1,
      Option::None,
    );
  }
  context_builder.start_with_context().await
}

pub async fn forward_slot(
  context: &mut ProgramTestContext,
  slot_count: u64,
) {
  let clock= context.banks_client.get_sysvar::<Clock>()
    .await.unwrap();
  let new_slot = clock.slot.checked_add(slot_count)
    .unwrap();
  context.warp_to_slot(new_slot).unwrap();
}

pub async fn forward_timestamp(
  context: &mut ProgramTestContext,
  timestamp_count: i64,
) {
  let clock= context.banks_client.get_sysvar::<Clock>()
    .await.unwrap();
  let mut new_clock = clock.clone();
  new_clock.unix_timestamp = clock.unix_timestamp.checked_add(timestamp_count)
    .unwrap();
  context.set_sysvar(&new_clock);
}



pub async fn process_transaction(
  context: &mut ProgramTestContext,
  payer: &Keypair,
  instructions: &[Instruction],
  signers: &[&Keypair],
) {
  let tx = Transaction::new_signed_with_payer(
    &instructions.to_vec(),
    Some(&payer.pubkey()),
    &signers.to_vec(),
    context.last_blockhash,
  );

  context.banks_client.process_transaction(tx)
    .await.unwrap();
}

pub async fn process_transaction2(
  context: &mut ProgramTestContext,
  payer: &Keypair,
  instructions: &[Instruction],
  signers: &[&Keypair],
) -> Result<(), BanksClientError> {
  let tx = Transaction::new_signed_with_payer(
    &instructions.to_vec(),
    Some(&payer.pubkey()),
    &signers.to_vec(),
    context.last_blockhash,
  );

  let txn_state = context.banks_client.process_transaction(tx)
    .await;
  txn_state
}
