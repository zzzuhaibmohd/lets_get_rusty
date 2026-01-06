mod eligibility; //@note -> mod is used to declare a cartain module exists and compile it into the build
mod input;
mod merkle;
mod types;

use eligibility::filter_eligible_users; //@note -> use th imported function from the imported module
use input::read_users_from_csv;
use merkle::{
    build_merkle_tree, create_merkle_tree, find_leaf_index, generate_proof, merkle_root,
    verify_proof,
};

fn main() {
    let users = read_users_from_csv("data/users.csv").expect("Failed to read users");

    let eligible_users = filter_eligible_users(users);

    let leaves = create_merkle_tree(eligible_users);

    let tree = build_merkle_tree(&leaves);

    let root = merkle_root(&tree);

    let target = &leaves[0];
    let index = find_leaf_index(&leaves, &target.address, target.amount);

    let proof = generate_proof(&tree, index.unwrap());

    let is_valid = verify_proof(&target.hash, &proof, &root);

    println!(
        "User: {} Amount: {} Is valid: {}",
        target.address, target.amount, is_valid
    );
}
