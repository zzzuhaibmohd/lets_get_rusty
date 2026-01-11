use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Challenge {
    pub id: u64,
    pub seed: String,
    pub difficulty_prefix: String,
    pub solved: bool,
    pub solution_nonce: Option<u64>,
    pub solution_hash: Option<String>, //@note Option<> is being used, since these values are not set at start and later updated. init to None
}
