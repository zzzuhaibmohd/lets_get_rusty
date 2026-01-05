use sha2::{Digest, Sha256};

use crate::types::{EligibleUser, MerkleLeaf, MerkleTree, UserRecord};

fn hash_bytes(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

pub fn create_merkle_tree(users: Vec<EligibleUser>) -> Vec<MerkleLeaf> {
    users
        .into_iter() //@note -> use into_iter() when not need to use the original vector again
        .map(|user| {
            let mut bytes = Vec::new();
            bytes.extend_from_slice(user.address.as_bytes());
            bytes.extend_from_slice(&user.amount.to_le_bytes());
            let hash = hash_bytes(&bytes);
            MerkleLeaf {
                address: user.address,
                amount: user.amount,
                hash,
            }
        })
        .collect() //@note -> collect() to convert the iterator into a vector
}

fn hash_pair(left: &[u8], right: &[u8]) -> Vec<u8> {
    let mut combined = Vec::new();

    combined.extend_from_slice(left);
    combined.extend_from_slice(right);
    hash_bytes(&combined)
}

fn build_next_level(current_level: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut next = Vec::new();

    let mut i = 0;

    while i < current_level.len() {
        let left = &current_level[i];
        let right = if i + 1 < current_level.len() {
            &current_level[i + 1]
        } else {
            left
        };
        next.push(hash_pair(left, right));
        i += 2;
    }
    next
}

pub fn build_merkle_tree(leaves: &[MerkleLeaf]) -> MerkleTree {
    assert!(!leaves.is_empty(), "Cannot build tree with no leaves");

    let mut levels: Vec<Vec<Vec<u8>>> = Vec::new(); //@note -> 3 dimensional vector to store L1 -- L2 -- L3(root)

    // Level 0 → leaf hashes
    let mut current_level: Vec<Vec<u8>> = leaves.iter().map(|l| l.hash.clone()).collect();

    levels.push(current_level.clone());

    // Build upwards
    while current_level.len() > 1 {
        current_level = build_next_level(&current_level);
        levels.push(current_level.clone());
    }

    MerkleTree { levels }
}

pub fn merkle_root(tree: &MerkleTree) -> Vec<u8> {
    tree.levels.last().expect("Tree has no levels")[0].clone()
}
