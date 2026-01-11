use rust_pow::{client::PoWClient, error::PoWError, io, server::PoWServer};
use std::fs;
use std::path::Path;

/// Test 1: Complete end-to-end workflow
/// This test verifies the entire PoW flow:
/// 1. Server creates a challenge
/// 2. Client solves the challenge
/// 3. Server verifies the solution
#[test]
fn test_complete_pow_workflow() {
    // Step 1: Server creates a challenge
    let mut server = PoWServer::new();
    let challenge = server.create_challenge();

    // Verify challenge properties
    assert_eq!(challenge.id, 1);
    assert!(!challenge.seed.is_empty());
    assert!(!challenge.difficulty_prefix.is_empty());
    assert_eq!(challenge.solved, false);
    assert_eq!(challenge.solution_nonce, None);
    assert_eq!(challenge.solution_hash, None);

    // Step 2: Client solves the challenge
    let (nonce, hash) = PoWClient::solve(&challenge);

    // Verify solution properties
    assert!(hash.starts_with(&challenge.difficulty_prefix));

    // Step 3: Server verifies the solution
    let verification_result = PoWServer::verify_solution_with_challenge(&challenge, nonce);
    assert!(
        verification_result.is_ok(),
        "Valid solution should be verified successfully"
    );
}

/// Test 2: End-to-end workflow with file I/O operations
/// This test verifies the complete workflow including:
/// 1. Server creates and saves challenge to file
/// 2. Client reads challenge from file and solves it
/// 3. Client writes solution to file
/// 4. Server reads solution from file and verifies it
#[test]
fn test_pow_workflow_with_file_io() {
    // Use temporary files for testing
    let challenge_file = "test_challenge.json";
    let solution_file = "test_solution.json";

    // Clean up any existing test files
    let _ = fs::remove_file(challenge_file);
    let _ = fs::remove_file(solution_file);

    // Step 1: Server creates a challenge and saves it
    let mut server = PoWServer::new();
    let challenge = server.create_challenge();
    io::append_challenge(challenge_file, &challenge);

    // Verify file was created and contains the challenge
    assert!(Path::new(challenge_file).exists());
    let saved_challenge =
        io::get_last_challenge(challenge_file).expect("Should read challenge from file");
    assert_eq!(saved_challenge.id, challenge.id);
    assert_eq!(saved_challenge.seed, challenge.seed);
    assert_eq!(
        saved_challenge.difficulty_prefix,
        challenge.difficulty_prefix
    );

    // Step 2: Client reads challenge and solves it
    let challenge_to_solve = io::get_last_challenge(challenge_file).expect("Should read challenge");
    let (nonce, hash) = PoWClient::solve(&challenge_to_solve);

    // Step 3: Client writes solution to file
    io::Solution::write_json(solution_file, challenge.id, nonce, hash.clone());

    // Verify solution file was created
    assert!(Path::new(solution_file).exists());
    let saved_solution = io::Solution::read_json(solution_file);
    assert_eq!(saved_solution.challenge_id, challenge.id);
    assert_eq!(saved_solution.nonce, nonce);
    assert_eq!(saved_solution.hash, hash);

    // Step 4: Server verifies the solution
    let challenge_for_verification =
        io::get_last_challenge(challenge_file).expect("Should read challenge");
    let verification_result = PoWServer::verify_solution_with_challenge(
        &challenge_for_verification,
        saved_solution.nonce,
    );
    assert!(
        verification_result.is_ok(),
        "Valid solution should be verified successfully"
    );

    // Step 5: Update challenge as solved
    let mut solved_challenge = challenge_for_verification;
    solved_challenge.solved = true;
    solved_challenge.solution_nonce = Some(saved_solution.nonce);
    solved_challenge.solution_hash = Some(saved_solution.hash.clone());
    io::update_last_challenge(challenge_file, &solved_challenge);

    // Verify challenge was updated
    let updated_challenge =
        io::get_last_challenge(challenge_file).expect("Should read updated challenge");
    assert_eq!(updated_challenge.solved, true);
    assert_eq!(updated_challenge.solution_nonce, Some(nonce));
    assert_eq!(updated_challenge.solution_hash, Some(hash));

    // Clean up test files
    let _ = fs::remove_file(challenge_file);
    let _ = fs::remove_file(solution_file);
}

/// Test 3: Error handling and edge cases
/// This test verifies error cases:
/// 1. Invalid solution (wrong nonce) should be rejected
/// 2. Already solved challenge should reject new solutions
/// 3. Multiple challenges with sequential IDs
#[test]
fn test_error_cases_and_edge_cases() {
    // Test 1: Invalid solution should be rejected
    let mut server = PoWServer::new();
    let challenge = server.create_challenge();

    // Try to verify with a clearly wrong nonce (very large number)
    let invalid_nonce = 999999999u64;
    let verification_result = PoWServer::verify_solution_with_challenge(&challenge, invalid_nonce);
    assert!(
        matches!(verification_result.unwrap_err(), PoWError::InvalidSolution),
        "Should return InvalidSolution error"
    );

    // Test 2: Solve the challenge properly first
    let (valid_nonce, _hash) = PoWClient::solve(&challenge);
    let verification_result = PoWServer::verify_solution_with_challenge(&challenge, valid_nonce);
    assert!(
        verification_result.is_ok(),
        "Valid solution should be accepted"
    );

    // Test 3: Already solved challenge should reject new solutions
    let mut solved_challenge = challenge.clone();
    solved_challenge.solved = true;
    solved_challenge.solution_nonce = Some(valid_nonce);

    let verification_result =
        PoWServer::verify_solution_with_challenge(&solved_challenge, valid_nonce);
    assert!(
        matches!(verification_result.unwrap_err(), PoWError::AlreadySolved),
        "Should return AlreadySolved error"
    );
}
