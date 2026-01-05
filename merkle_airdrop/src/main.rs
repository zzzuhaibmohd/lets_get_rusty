mod eligibility; //@note -> mod is used to declare a cartain module exists and compile it into the build
mod input;
mod types;

use eligibility::filter_eligible_users; //@note -> use th imported function from the imported module
use input::read_users_from_csv;
use types::{EligibleUser, UserRecord};

fn main() {
    let users = read_users_from_csv("data/users.csv").expect("Failed to read users");

    let eligible_users = filter_eligible_users(users);

    println!("{:?}", eligible_users);
    println!("Total eligible users: {}", eligible_users.len());
}
