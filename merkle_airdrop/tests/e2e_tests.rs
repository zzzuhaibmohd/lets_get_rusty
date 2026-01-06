use merkle_airdrop::eligibility::filter_eligible_users;
use merkle_airdrop::merkle::{
    build_merkle_tree, create_merkle_tree, find_leaf_index, generate_proof, merkle_root,
    verify_proof,
};
use merkle_airdrop::types::{EligibleUser, UserRecord};
use sha2::{Digest, Sha256};

#[test]
fn test_valid_proof_generation() {
    // Setup: Create test users with eligible volumes
    let users = vec![
        UserRecord {
            address: "0x1111111111111111111111111111111111111111".to_string(),
            total_volume: 150,
        },
        UserRecord {
            address: "0x2222222222222222222222222222222222222222".to_string(),
            total_volume: 200,
        },
        UserRecord {
            address: "0x3333333333333333333333333333333333333333".to_string(),
            total_volume: 300,
        },
    ];

    // Step 1: Filter eligible users
    let eligible_users = filter_eligible_users(users);
    assert_eq!(eligible_users.len(), 3, "All users should be eligible");

    // Step 2: Create merkle tree leaves
    let leaves = create_merkle_tree(eligible_users);

    // Step 3: Build merkle tree
    let tree = build_merkle_tree(&leaves);

    // Step 4: Get merkle root
    let root = merkle_root(&tree);

    // Step 5: Generate proof for the first user
    let target_address = "0x1111111111111111111111111111111111111111";
    let target_amount = 150u64;
    let target_index = find_leaf_index(&leaves, target_address, target_amount)
        .expect("Target user should be found in leaves");

    let target_leaf = &leaves[target_index];
    let proof = generate_proof(&tree, target_index);

    // Step 6: Verify the proof
    let is_valid = verify_proof(&target_leaf.hash, &proof, &root);

    assert!(
        is_valid,
        "Proof should be valid for correct user and amount"
    );
}

#[test]
fn test_invalid_proof_with_wrong_amount() {
    // Setup: Create test users with eligible volumes
    let users = vec![
        UserRecord {
            address: "0x1111111111111111111111111111111111111111".to_string(),
            total_volume: 150,
        },
        UserRecord {
            address: "0x2222222222222222222222222222222222222222".to_string(),
            total_volume: 200,
        },
    ];

    // Step 1: Filter eligible users
    let eligible_users = filter_eligible_users(users);
    assert_eq!(eligible_users.len(), 2, "Both users should be eligible");

    // Step 2: Create merkle tree leaves
    let leaves = create_merkle_tree(eligible_users);

    // Step 3: Build merkle tree
    let tree = build_merkle_tree(&leaves);

    // Step 4: Get merkle root
    let root = merkle_root(&tree);

    // Step 5: Generate proof for the first user (with correct amount 150)
    let target_address = "0x1111111111111111111111111111111111111111";
    let correct_amount = 150u64;
    let target_index = find_leaf_index(&leaves, target_address, correct_amount)
        .expect("Target user should be found in leaves");

    let proof = generate_proof(&tree, target_index);

    // Step 6: Try to verify with wrong amount (claiming 300 instead of 150)
    // Create a fake leaf hash with the wrong amount
    let mut bytes = Vec::new();
    bytes.extend_from_slice(target_address.as_bytes());
    bytes.extend_from_slice(&300u64.to_le_bytes()); // Wrong amount!
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let fake_leaf_hash = hasher.finalize().to_vec();

    // Step 7: Verify the proof with incorrect leaf hash
    let is_valid = verify_proof(&fake_leaf_hash, &proof, &root);

    assert!(
        !is_valid,
        "Proof should be invalid when claiming wrong amount"
    );
}
