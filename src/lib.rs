use sha2::{Digest, Sha256};

const ZERO_BYTES: [u8; 32] = [0; 32];

pub struct MerkleTree {
    pub leaves: Vec<[u8; 32]>,
}

impl MerkleTree {
    pub fn new() -> Self {
        Self { leaves: Vec::new() }
    }

    pub fn from_slice(data: &[[u8; 32]]) -> Self {
        let mut leaves = data.to_vec();
        let cap: usize = leaves.len().next_power_of_two();

        leaves.resize(cap, ZERO_BYTES);

        Self { leaves }
    }

    pub fn root(&self) -> [u8; 32] {
        if self.leaves.is_empty() {
            return ZERO_BYTES;
        }

        if self.leaves.len() == 1 {
            return self.leaves[0];
        }

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

impl Default for MerkleTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_slice_empty() {
        let tree = MerkleTree::from_slice(&[]);

        assert_eq!(tree.leaves.len(), 1);
        assert_eq!(tree.leaves[0], ZERO_BYTES);
    }

    #[test]
    fn from_slice_single_leaf() {
        let data = vec![[1; 32]];
        let tree = MerkleTree::from_slice(&data);

        assert_eq!(tree.leaves.len(), 1);
        assert_eq!(tree.leaves[0], data[0]);
    }

    #[test]
    fn from_slice_two_leaves() {
        let data = vec![[1; 32], [2; 32]];
        let tree = MerkleTree::from_slice(&data);

        assert_eq!(tree.leaves.len(), 2);
        assert_eq!(tree.leaves[0], data[0]);
        assert_eq!(tree.leaves[1], data[1]);
    }

    #[test]
    fn from_slice_three_leaves_pads_to_four() {
        let data = [[1; 32], [2; 32], [3; 32]];
        let tree = MerkleTree::from_slice(&data);

        assert_eq!(tree.leaves.len(), 4);
        assert_eq!(tree.leaves[3], ZERO_BYTES);
    }

    #[test]
    fn from_slice_five_leaves_pads_to_eight() {
        let data = [[1; 32], [2; 32], [3; 32], [4; 32], [5; 32]];
        let tree = MerkleTree::from_slice(&data);

        assert_eq!(tree.leaves.len(), 8);
        assert_eq!(tree.leaves[5], ZERO_BYTES);
        assert_eq!(tree.leaves[6], ZERO_BYTES);
        assert_eq!(tree.leaves[7], ZERO_BYTES);
    }

    #[test]
    fn root_with_no_leaves_return_zero_bytes() {
        let tree = MerkleTree::from_slice(&[]);

        let root = tree.root();
        assert_eq!(root, ZERO_BYTES);
    }

    #[test]
    fn root_with_single_leaf_return_that_leaf() {
        let data = vec![[1; 32]];
        let tree = MerkleTree::from_slice(&data);

        let root = tree.root();
        assert_eq!(root, data[0]);
    }

    #[test]
    fn root_with_two_leaves_hashes_correctly() {
        let data = [[1; 32], [2; 32]];
        let tree = MerkleTree::from_slice(&data);

        let expected = Sha256::digest([data[0], data[1]].concat());
        assert_eq!(tree.root(), expected.as_slice());
    }

    #[test]
    fn root_with_four_leaves() {
        let data = [[1; 32], [2; 32], [3; 32], [4; 32]];
        let tree = MerkleTree::from_slice(&data);

        let h01 = Sha256::digest([data[0], data[1]].concat());
        let h23 = Sha256::digest([data[2], data[3]].concat());
        let expected = Sha256::digest([h01, h23].concat());

        assert_eq!(tree.root(), expected.as_slice());
    }

    #[test]
    fn root_with_three_leaves_pads_and_hashes() {
        let data = [[1; 32], [2; 32], [3; 32]];
        let tree = MerkleTree::from_slice(&data);

        let h01 = Sha256::digest([data[0], data[1]].concat());
        let h23 = Sha256::digest([data[2], ZERO_BYTES].concat());
        let expected = Sha256::digest([h01, h23].concat());

        assert_eq!(tree.root(), expected.as_slice());
    }
}
