use crate::challenge::Challenge;
use crate::hash::sha256_hex;

pub struct PoWClient {}

impl PoWClient {
    pub fn solve(challenge: &Challenge) -> (u64, String) {
        let mut nonce = 0u64;

        loop {
            let input = format!("{}{}", challenge.seed, nonce);
            let hash = sha256_hex(&input);
            if hash.starts_with(&challenge.difficulty_prefix) {
                return (nonce, hash);
            }
            nonce += 1;
        }
    }
}
