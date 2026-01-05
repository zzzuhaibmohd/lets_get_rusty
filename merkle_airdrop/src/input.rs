use std::error::Error;
use std::fs;

use crate::types::UserRecord;

pub fn read_users_from_csv(path: &str) -> Result<Vec<UserRecord>, Box<dyn Error>> {
    //@note -> Box<dyn Error> is used to handle most common error types
    let contents = fs::read_to_string(path)?;

    let mut users = Vec::new();

    for (i, line) in contents.lines().enumerate() {
        if i == 0 {
            continue; //skip header
        }

        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 2 {
            continue; //skip invalid line
        }

        let address = parts[0].to_string();
        let total_volume = parts[1].parse::<u64>()?;

        users.push(UserRecord {
            address,
            total_volume,
        });
    }
    Ok(users)
}
