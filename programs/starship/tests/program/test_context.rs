use solana_program_test::{
  ProgramTestContext,
};
use crate::framework::{
  context::{
    create_context,
  }
};
use coin98_starship::{
  ID as PROGRAM_ID,
};

pub async fn create_test_context(
) -> ProgramTestContext {
  let context = create_context(
    &[
      ("coin98_starship", PROGRAM_ID)
    ]
  ).await;
  context
}
