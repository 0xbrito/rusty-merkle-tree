use sha2::{Digest, Sha256};

const ZERO_BYTES: [u8; 32] = [0; 32];

pub struct MerkleTree {
    pub leaves: Vec<[u8; 32]>,
    pub count: usize,
}

impl MerkleTree {
    pub fn new() -> Self {
        Self::from_slice(&[])
    }

    pub fn from_slice(data: &[[u8; 32]]) -> Self {
        let leaves_len = data.len();
        let cap: usize = leaves_len.next_power_of_two();

        let mut leaves = vec![ZERO_BYTES; cap];

        leaves[..leaves_len].copy_from_slice(data);

        for (i, leaf) in data.iter().enumerate() {
            leaves[i] = *leaf;
        }

        Self {
            leaves,
            count: leaves_len,
        }
    }

    pub fn insert(&mut self, new_leaf: [u8; 32]) {
        let current_len_with_padding = self.leaves.len();

        if self.count == current_len_with_padding {
            // expand cap and pad with zeroes
            let new_cap = (current_len_with_padding + 1).next_power_of_two();
            let mut leaves_new = vec![ZERO_BYTES; new_cap];

            leaves_new[..self.leaves.len()].copy_from_slice(&self.leaves);

            self.leaves = leaves_new;
        }

        let new_leaf_index = self.count;
        self.leaves[new_leaf_index] = new_leaf;
        self.count += 1;
    }

    pub fn root(&self) -> [u8; 32] {
        let mut level = self.leaves.clone();

        while level.len() > 1 {
            let pair_count: usize = level.len() / 2;
            let mut next_level = Vec::with_capacity(pair_count);
            for i in 0..pair_count {
                let left = level[i * 2];
                let right = level[i * 2 + 1];

                let hash: [u8; 32] = Self::hash_pair(left, right);
                next_level.push(hash);
            }
            level = next_level;
        }
        level[0]
    }

    pub fn verify(&self, leaf: [u8; 32], index: usize, proof: &[[u8; 32]]) -> bool {
        let mut hash = leaf;

        for (level, sibling) in proof.iter().enumerate() {
            let (left, right) = if (index >> level) & 1 == 0 {
                (&hash, sibling)
            } else {
                (sibling, &hash)
            };

            hash = Self::hash_pair(*left, *right);
        }

        hash == self.root()
    }

    pub fn proof_for(&self, mut index: usize) -> Vec<[u8; 32]> {
        let mut proof = Vec::new();
        let mut level = self.leaves.clone();

        while level.len() > 1 {
            proof.push(level[index ^ 1]);

            let pair_count: usize = level.len() / 2;
            let mut next_level = Vec::with_capacity(pair_count);
            for i in 0..pair_count {
                let left = level[i * 2];
                let right = level[i * 2 + 1];

                let hash: [u8; 32] = Self::hash_pair(left, right);
                next_level.push(hash);
            }
            level = next_level;

            index /= 2;
        }

        proof
    }

    fn hash_pair(left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
        Sha256::digest([left, right].concat()).into()
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
    use sha2::{Digest, Sha256};

    fn hash(data: &[u8]) -> [u8; 32] {
        Sha256::digest(data).into()
    }

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
    fn insert_full_tree_expands_to_a_power_of_two_and_pads_zeroes() {
        let data = [[1; 32], [2; 32]];
        let mut tree = MerkleTree::from_slice(&data);

        let new_leaf = [3; 32];

        assert_eq!(tree.leaves.len(), 2);

        let root_before = tree.root();

        tree.insert(new_leaf);

        assert_ne!(tree.root(), root_before);
        assert_eq!(tree.leaves.len(), 4);
        assert_eq!(tree.count, 3);
        assert_eq!(tree.leaves[2], new_leaf);
        assert_eq!(tree.leaves[3], ZERO_BYTES);
    }

    #[test]
    fn insert_into_tree_with_space() {
        let data = [[1; 32], [2; 32], [3; 32], [4; 32], [5; 32]];
        let mut tree = MerkleTree::from_slice(&data);

        let new_leaf = [6; 32];

        assert_eq!(tree.leaves.len(), 8);

        let root_before = tree.root();

        tree.insert(new_leaf);

        assert_ne!(tree.root(), root_before);
        assert_eq!(tree.leaves.len(), 8);
        assert_eq!(tree.count, 6);
        assert_eq!(tree.leaves[5], new_leaf);
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

    #[test]
    fn verify_valid_proof() {
        let a = hash(b"a");
        let b = hash(b"b");
        let c = hash(b"c");
        let d = hash(b"d");
        let tree = MerkleTree::from_slice(&[a, b, c, d]);

        let leaf = b;
        let proof = [a, hash(&[c, d].concat())];

        assert!(tree.verify(leaf, 1, &proof));
    }

    #[test]
    fn verify_single_leaf_empty_proof() {
        let a = hash(b"a");
        let tree = MerkleTree::from_slice(&[a]);

        assert!(tree.verify(a, 0, &[]));
    }

    #[test]
    fn verify_wrong_leaf_returns_false() {
        let a = hash(b"a");
        let b = hash(b"b");
        let c = hash(b"c");
        let d = hash(b"d");
        let tree = MerkleTree::from_slice(&[a, b, c, d]);

        let leaf = hash(b"wrong");
        let proof = [a, hash(&[c, d].concat())];

        assert!(!tree.verify(leaf, 1, &proof));
    }

    #[test]
    fn verify_tampered_proof_returns_false() {
        let a = hash(b"a");
        let b = hash(b"b");
        let c = hash(b"c");
        let d = hash(b"d");
        let tree = MerkleTree::from_slice(&[a, b, c, d]);

        let leaf = b;
        let tampered_sibling = [0xFF; 32];
        let proof = [tampered_sibling, hash(&[c, d].concat())];

        assert!(!tree.verify(leaf, 1, &proof));
    }

    #[test]
    fn proof_for_single_leaf_returns_empty() {
        let tree = MerkleTree::new();

        let proof = tree.proof_for(0);

        assert_eq!(proof, Vec::<[u8; 32]>::from([]));
    }

    #[test]
    fn proof_for_two_leaves() {
        let a = hash(b"a");
        let b = hash(b"b");
        let tree = MerkleTree::from_slice(&[a, b]);

        let proof = tree.proof_for(0);

        assert_eq!(proof, vec![b]);
    }

    #[test]
    fn proof_for_four_leaves_index_zero() {
        let a = hash(b"a");
        let b = hash(b"b");
        let c = hash(b"c");
        let d = hash(b"d");
        let tree = MerkleTree::from_slice(&[a, b, c, d]);

        let proof = tree.proof_for(0);

        assert_eq!(proof, vec![b, hash(&[c, d].concat())]);
    }

    #[test]
    fn proof_for_four_leaves_index_three() {
        let a = hash(b"a");
        let b = hash(b"b");
        let c = hash(b"c");
        let d = hash(b"d");
        let tree = MerkleTree::from_slice(&[a, b, c, d]);

        let proof = tree.proof_for(3);

        assert_eq!(proof, vec![c, hash(&[a, b].concat())]);
    }
}
