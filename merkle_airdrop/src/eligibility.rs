use crate::types::{EligibleUser, UserRecord};

const MIN_VOLUME: u64 = 100;
const MULTIPLIER: u64 = 1;

pub fn filter_eligible_users(users: Vec<UserRecord>) -> Vec<EligibleUser> {
    users
        .into_iter()
        .filter(|user| user.total_volume >= MIN_VOLUME)
        .map(|user| EligibleUser {
            address: user.address.clone(),
            amount: user.total_volume * MULTIPLIER,
        })
        .collect()
}
