use crate::challenge::Challenge;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::fs;
use std::path::Path;

// ============================================================================
// Solution
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Solution {
    pub challenge_id: u64,
    pub nonce: u64,
    pub hash: String,
}

impl Solution {
    pub fn write_json(path: &str, challenge_id: u64, nonce: u64, hash: String) {
        let solution = Solution {
            challenge_id,
            nonce,
            hash,
        };
        JsonFile::write(path, &solution);
    }

    pub fn read_json(path: &str) -> Solution {
        JsonFile::read(path)
    }
}

// ============================================================================
// Challenge History
// ============================================================================

pub struct ChallengeHistory;

impl ChallengeHistory {
    pub fn read(path: &str) -> Vec<Challenge> {
        if !Path::new(path).exists() {
            return Vec::new();
        }

        let json = fs::read_to_string(path).unwrap();
        serde_json::from_str::<Vec<Challenge>>(&json).unwrap_or_default()
    }

    pub fn write(path: &str, challenges: &[Challenge]) {
        let json = serde_json::to_string_pretty(challenges).unwrap();
        fs::write(path, json).unwrap();
    }

    pub fn get_last(path: &str) -> Option<Challenge> {
        Self::read(path).last().cloned()
    }

    pub fn append(path: &str, challenge: &Challenge) {
        let mut history = Self::read(path);
        history.push(challenge.clone());
        Self::write(path, &history);
    }

    pub fn update_last(path: &str, updated_challenge: &Challenge) {
        let mut history = Self::read(path);
        if let Some(last) = history.last_mut() {
            *last = updated_challenge.clone();
        }
        Self::write(path, &history);
    }
}

// ============================================================================
// Generic JSON File Operations
// ============================================================================

pub struct JsonFile;

impl JsonFile {
    /// Write any serializable type to a JSON file
    pub fn write<T: Serialize>(path: &str, data: &T) {
        let json = serde_json::to_string_pretty(data).unwrap();
        fs::write(path, json).unwrap();
    }

    /// Read any deserializable type from a JSON file
    pub fn read<T: DeserializeOwned>(path: &str) -> T {
        let json = fs::read_to_string(path).unwrap();
        serde_json::from_str(&json).unwrap()
    }
}

// ============================================================================
// Wrapper Functions
// ============================================================================

pub fn read_json<T: DeserializeOwned>(path: &str) -> T {
    JsonFile::read(path)
}

pub fn get_last_challenge(path: &str) -> Option<Challenge> {
    ChallengeHistory::get_last(path)
}

pub fn append_challenge(path: &str, challenge: &Challenge) {
    ChallengeHistory::append(path, challenge);
}

pub fn update_last_challenge(path: &str, updated_challenge: &Challenge) {
    ChallengeHistory::update_last(path, updated_challenge);
}
