use anchor_lang::{
  AnchorSerialize,
  prelude::*,
};
use solana_sdk::{
  keccak::{
    hashv,
  },
  pubkey::{
    Pubkey,
  },
};
use crate::{
  framework::{
    merkle_tree::{
      MerkleTree,
    },
  },
};

#[derive(AnchorSerialize)]
pub struct WhiteListParams {
  pub index: u32,
  pub address: Pubkey,
}

pub fn create_whilelist_tree(
  addresses: &[Pubkey],
) -> MerkleTree {
  let mut hashes: Vec<Vec<u8>> = Vec::new();
  for i in 0..addresses.len() {
    let params = WhiteListParams {
      index: u32::try_from(i).unwrap(),
      address: addresses[i],
    };
    let params_bytes = params.try_to_vec().unwrap();
    let hash = hashv(&[&params_bytes[..]])
      .to_bytes()
      .to_vec();
    hashes.push(hash);
  }

  MerkleTree::create_from_bytes(hashes)
}
