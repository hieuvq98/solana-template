use solana_sdk::{
  keccak::{
    hashv,
  },
};

const LEVEL_ARRAY: &[char] = &['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M'];
const SIZE_ARRAY: &[usize] = &[1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096];

pub struct MerkleNode {
  pub row: char,
  pub index: u64,
  pub hash: Vec<u8>,
}

impl Clone for MerkleNode {
  fn clone(&self) -> MerkleNode {
    MerkleNode {
      row: self.row,
      index: self.index,
      hash: self.hash.clone(),
    }
  }
}

pub struct MerkleTree {
  pub height: usize,
  pub nodes: Vec<Vec<MerkleNode>>,
  pub root: MerkleNode,
}

impl MerkleTree {
  pub fn create_from_bytes(
    hashes: Vec<Vec<u8>>
  ) -> MerkleTree {
    let mut height = 0usize;
    for i in 0..SIZE_ARRAY.len() {
      height += 1;
      if SIZE_ARRAY[i] >= hashes.len() {
        break;
      }
    }

    let mut leaf_nodes: Vec<MerkleNode> = Vec::new();
    let empty_hash = vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    let leaf_length = SIZE_ARRAY[height];
    for i in 0..leaf_length {
      let hash = if i >= hashes.len() { empty_hash.clone() } else { hashes[i].clone() };
      let leaf = MerkleNode {
        row: LEVEL_ARRAY[0],
        index: u64::try_from(i).unwrap(),
        hash,
      };
      leaf_nodes.push(leaf);
    }

    let mut nodes: Vec<Vec<MerkleNode>> = Vec::new();
    nodes.push(leaf_nodes);

    for i in 1..height {
      let sub_nodes = &nodes[i-1];
      let mut new_nodes: Vec<MerkleNode> = Vec::new();
      for j in (0..sub_nodes.len()).step_by(2) {
        let hash_0 = &sub_nodes[j].hash;
        let hash_1 = &sub_nodes[j+1].hash;
        let new_hash = keccak_two_hashes(hash_0, hash_1);
        let new_node = MerkleNode {
          row: LEVEL_ARRAY[i],
          index: u64::try_from(j/2).unwrap(),
          hash: new_hash,
        };
        new_nodes.push(new_node);
      }
      nodes.push(new_nodes);
    }

    let root = nodes[height-1][0].clone();

    MerkleTree {
      height,
      nodes,
      root,
    }
  }
}

fn keccak_two_hashes(
  x: &Vec<u8>,
  y: &Vec<u8>,
) -> Vec<u8> {
  let hash: [u8; 32] = if x < y {
    hashv(&[&x, &y]).to_bytes()
  } else {
    hashv(&[&y, &x]).to_bytes()
  };
  hash.to_vec()
}
