use rand::{Rng, thread_rng};

use crate::{challenge::Challenge, error::PoWError, hash::sha256_hex};

pub struct PoWServer {
    next_id: u64,
    challenges: Vec<Challenge>,
}

impl PoWServer {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            challenges: Vec::new(),
        }
    }

    pub fn with_next_id(next_id: u64) -> Self {
        Self {
            next_id,
            challenges: Vec::new(),
        }
    }

    pub fn create_challenge(&mut self) -> Challenge {
        let seed = thread_rng().r#gen::<u64>().to_string();
        let difficulty_prefix = format!("{:03}", thread_rng().gen_range(0..1000));

        let challenge = Challenge {
            id: self.next_id,
            seed,
            difficulty_prefix,
            solved: false,
            solution_nonce: None,
            solution_hash: None,
        };
        self.challenges.push(challenge.clone());
        self.next_id += 1;
        challenge
    }

    /// Verify a solution using a challenge directly (doesn't require server state)
    pub fn verify_solution_with_challenge(
        challenge: &Challenge,
        nonce: u64,
    ) -> Result<(), PoWError> {
        if challenge.solved {
            return Err(PoWError::AlreadySolved);
        }

        let input = format!("{}{}", challenge.seed, nonce);
        let hash = sha256_hex(&input);
        if hash.starts_with(&challenge.difficulty_prefix) {
            return Ok(());
        }
        Err(PoWError::InvalidSolution)
    }
}
