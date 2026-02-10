use sha2::{Digest, Sha256};

const ZERO_BYTES: [u8; 32] = [0; 32];

pub struct MerkleTree {
    leaves: Vec<[u8; 32]>,
}

impl MerkleTree {
    pub fn new(leaves: Vec<[u8; 32]>) -> Self {
        let leaves_capacity = leaves.len().next_power_of_two();
        let mut hashes = leaves;
        hashes.resize(leaves_capacity, ZERO_BYTES);
        Self { leaves: hashes }
    }

    fn compute_root(&self) -> [u8; 32] {
        let mut hashes = self.leaves.clone();
        while hashes.len() > 1 {
            let pair_count: usize = hashes.len() / 2;
            let mut next_level = Vec::with_capacity(pair_count);
            for i in 0..pair_count {
                let left = hashes[i * 2];
                let right = hashes[i * 2 + 1];

                let hash: [u8; 32] = Sha256::digest([left, right].concat())
                    .as_slice()
                    .try_into()
                    .unwrap();
                next_level.push(hash);
            }
            hashes = next_level;
        }
        hashes[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let res = MerkleTree(1, 2);
        // assert_eq!(res, 3);
    }
}
