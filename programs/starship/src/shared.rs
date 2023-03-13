use std::convert::{
  TryFrom,
};
use anchor_lang::prelude::{msg, Result};

use crate::error::ErrorCode;

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

pub fn calculate_out_total(
  amount: u64,
  price_n: u64,
  price_d: u64,
) -> Option<u64> {
  if amount == 0 {
    Some(0)
  } else {
    let x = u128::from(amount);
    let n = u128::from(price_n);
    let d = u128::from(price_d);
    let result = x
      .checked_mul(d)?
      .checked_div(n)?;
    let u64_max = u128::from(u64::MAX);
    if result > u64_max {
      None
    } else {
      Some(u64::try_from(result).unwrap())
    }
  }
}

pub fn calculate_system_fee(
  amount: u64,
  protocol_fee: u64,
  sharing_fee: u64,
) -> u64 {
  if amount == 0 {
    0
  } else {
    let mut system_fee: u64 = (amount.checked_mul(protocol_fee).unwrap()).checked_div(10000).unwrap();
    system_fee = system_fee.checked_add(sharing_fee).unwrap();
    system_fee
  }
}

pub fn check_ed25519_data(data: &[u8], pubkey: &[u8], msg: &[u8], sig: &[u8]) -> Result<()> {
  // https://github.com/solana-labs/solana-web3.js/blob/master/src/ed25519-program.ts#L33
  // "Deserializing" byte slices

  let num_signatures                  = &[data[0]];        // Byte  0
  let padding                         = &[data[1]];        // Byte  1
  let signature_offset                = &data[2..=3];      // Bytes 2,3
  let signature_instruction_index     = &data[4..=5];      // Bytes 4,5
  let public_key_offset               = &data[6..=7];      // Bytes 6,7
  let public_key_instruction_index    = &data[8..=9];      // Bytes 8,9
  let message_data_offset             = &data[10..=11];    // Bytes 10,11
  let message_data_size               = &data[12..=13];    // Bytes 12,13
  let message_instruction_index       = &data[14..=15];    // Bytes 14,15

  let data_pubkey                     = &data[16..16+32];  // Bytes 16..16+32
  let data_sig                        = &data[48..48+64];  // Bytes 48..48+64
  let data_msg                        = &data[112..];      // Bytes 112..end

  // Expected values

  let exp_public_key_offset:      u16 = 16; // 2*u8 + 7*u16
  let exp_signature_offset:       u16 = exp_public_key_offset + pubkey.len() as u16;
  let exp_message_data_offset:    u16 = exp_signature_offset + sig.len() as u16;
  let exp_num_signatures:          u8 = 1;
  let exp_message_data_size:      u16 = msg.len().try_into().unwrap();

  // Header and Arg Checks

  // Header
  if num_signatures                  != &exp_num_signatures.to_le_bytes()        ||
    padding                         != &[0]                                     ||
    signature_offset                != &exp_signature_offset.to_le_bytes()      ||
    signature_instruction_index     != &u16::MAX.to_le_bytes()                  ||
    public_key_offset               != &exp_public_key_offset.to_le_bytes()     ||
    public_key_instruction_index    != &u16::MAX.to_le_bytes()                  ||
    message_data_offset             != &exp_message_data_offset.to_le_bytes()   ||
    message_data_size               != &exp_message_data_size.to_le_bytes()     ||
    message_instruction_index       != &u16::MAX.to_le_bytes()  
  {
    return Err(ErrorCode::SigVerificationFailed.into());
  }

  // Arguments
  if data_pubkey != pubkey {
    msg!("invalid pubkey signer");
    return Err(ErrorCode::SigVerificationFailed.into());
  }
  if data_sig != sig {
    msg!("invalid signature");
    return Err(ErrorCode::SigVerificationFailed.into());
  }
  if data_msg != msg {
    msg!("invalid message signature");
    return Err(ErrorCode::SigVerificationFailed.into());
  }
  Ok(())
}