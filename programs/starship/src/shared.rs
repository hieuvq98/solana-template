use solana_program::{
  keccak::{
    hashv,
  },
};
use std::convert::{
  TryFrom,
};

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

/// Returns true if a `leaf` can be proved to be a part of a Merkle tree
/// defined by `root`. For this, a `proof` must be provided, containing
/// sibling hashes on the branch from the leaf to the root of the tree. Each
/// pair of leaves and each pair of pre-images are assumed to be sorted.
pub fn verify_proof(proofs: &Vec<[u8; 32]>, root: &[u8; 32], leaf: &[u8; 32]) -> bool {
  let mut computed_hash = *leaf;
  for proof in proofs.into_iter() {
    if computed_hash < *proof {
      // Hash(current computed hash + current element of the proof)
      computed_hash = hashv(&[&computed_hash, proof]).to_bytes();
    } else {
      // Hash(current element of the proof + current computed hash)
      computed_hash = hashv(&[proof, &computed_hash]).to_bytes();
    }
  }
  // Check if the computed hash (root) is equal to the provided root
  computed_hash == *root
}
