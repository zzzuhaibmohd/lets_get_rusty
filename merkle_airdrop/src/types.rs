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
