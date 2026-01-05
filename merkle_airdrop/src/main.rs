mod eligibility; //@note -> mod is used to declare a cartain module exists and compile it into the build
mod input;
mod merkle;
mod types;

use eligibility::filter_eligible_users; //@note -> use th imported function from the imported module
use input::read_users_from_csv;
use merkle::{build_merkle_tree, create_merkle_tree, merkle_root};
use types::{EligibleUser, MerkleLeaf, UserRecord};

fn main() {
    let users = read_users_from_csv("data/users.csv").expect("Failed to read users");

    let eligible_users = filter_eligible_users(users);

    let leaves = create_merkle_tree(eligible_users);

    let tree = build_merkle_tree(&leaves);

    let root = merkle_root(&tree);
}
