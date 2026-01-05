#[derive(Debug, Clone)]
pub struct UserRecord {
    pub address: String,
    pub total_volume: u64,
}

#[derive(Debug, Clone)]
pub struct EligibleUser {
    pub address: String,
    pub amount: u64,
}

#[derive(Debug, Clone)]
pub struct MerkleLeaf {
    pub address: String,
    pub amount: u64,
    pub hash: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub levels: Vec<Vec<Vec<u8>>>,
}
