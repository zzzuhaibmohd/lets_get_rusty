use clap::{Parser, Subcommand};
use std::path::Path;

use rust_pow::{challenge::Challenge, client::PoWClient, io, server::PoWServer};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Server {
        #[command(subcommand)]
        action: ServerCmd,
    },
    Client {
        #[command(subcommand)]
        action: ClientCmd,
    },
}

#[derive(Subcommand)]
enum ServerCmd {
    Create,
    Verify { solution_file: String },
}

#[derive(Subcommand)]
enum ClientCmd {
    Solve { challenge_file: String },
}

fn handle_io_error<T, F>(operation: F, error_msg: &str) -> T
where
    F: FnOnce() -> T,
{
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(operation)) {
        Ok(result) => result,
        Err(_) => {
            eprintln!("Error: {}", error_msg);
            std::process::exit(1);
        }
    }
}

fn get_last_challenge_safe(path: &str) -> Option<Challenge> {
    if Path::new(path).exists() {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            io::get_last_challenge(path)
        })) {
            Ok(challenge) => challenge,
            Err(_) => None,
        }
    } else {
        None
    }
}

fn get_last_challenge_or_exit(path: &str) -> Challenge {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        io::get_last_challenge(path)
    })) {
        Ok(Some(ch)) => ch,
        Ok(None) => {
            eprintln!("Error: No challenges found in {}", path);
            std::process::exit(1);
        }
        Err(_) => {
            eprintln!("Error: Failed to read challenge file '{}'", path);
            std::process::exit(1);
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let mut server = PoWServer::new();

    match cli.command {
        Commands::Server { action } => match action {
            ServerCmd::Create => {
                // Check if there's an existing unsolved challenge
                if let Some(existing_challenge) = get_last_challenge_safe("challenge.json") {
                    if !existing_challenge.solved {
                        eprintln!(
                            "Error: Unsolved challenge (ID: {}) exists in challenge.json. Only one challenge should be running at once. Please solve the current challenge before creating a new one.",
                            existing_challenge.id
                        );
                        std::process::exit(1);
                    }
                    // If solved, use the next ID after the last challenge
                    server = PoWServer::with_next_id(existing_challenge.id + 1);
                }

                let challenge = server.create_challenge();
                handle_io_error(
                    || io::append_challenge("challenge.json", &challenge),
                    "Failed to append challenge to 'challenge.json'",
                );
                println!(
                    "Challenge created! ID: {}, Seed: {}, Difficulty: {}",
                    challenge.id, challenge.seed, challenge.difficulty_prefix
                );
            }
            ServerCmd::Verify { solution_file } => {
                let solution = handle_io_error(
                    || io::Solution::read_json(&solution_file),
                    &format!("Failed to read solution file '{}'", solution_file),
                );

                let mut challenge = get_last_challenge_or_exit("challenge.json");

                if challenge.id != solution.challenge_id {
                    eprintln!(
                        "Error: Challenge ID mismatch. Solution is for challenge {}, but last challenge in file has ID {}",
                        solution.challenge_id, challenge.id
                    );
                    std::process::exit(1);
                }

                match PoWServer::verify_solution_with_challenge(&challenge, solution.nonce) {
                    Ok(()) => {
                        // Mark challenge as solved and save solution info
                        challenge.solved = true;
                        challenge.solution_nonce = Some(solution.nonce);
                        challenge.solution_hash = Some(solution.hash.clone());

                        handle_io_error(
                            || io::update_last_challenge("challenge.json", &challenge),
                            "Failed to update challenge file 'challenge.json'",
                        );

                        println!(
                            "Solution verified! Challenge ID: {}, Nonce: {}, Hash: {}",
                            solution.challenge_id, solution.nonce, solution.hash
                        );
                    }
                    Err(e) => {
                        let input = format!("{}{}", challenge.seed, solution.nonce);
                        let computed_hash = rust_pow::hash::sha256_hex(&input);
                        eprintln!(
                            "Error: Verification failed: {}. Expected prefix: {}, Computed hash: {}, Solution hash: {}",
                            e, challenge.difficulty_prefix, computed_hash, solution.hash
                        );
                        std::process::exit(1);
                    }
                }
            }
        },
        Commands::Client { action } => match action {
            ClientCmd::Solve { challenge_file } => {
                // If reading from challenge.json, get the last challenge from history
                // Otherwise, read as a single challenge file (for backward compatibility)
                let challenge: Challenge = if challenge_file == "challenge.json" {
                    get_last_challenge_or_exit(&challenge_file)
                } else {
                    handle_io_error(
                        || io::read_json(&challenge_file),
                        &format!("Failed to read challenge file '{}'", challenge_file),
                    )
                };

                let (nonce, hash) = PoWClient::solve(&challenge);

                handle_io_error(
                    || io::Solution::write_json("solution.json", challenge.id, nonce, hash.clone()),
                    "Failed to write solution file 'solution.json'",
                );

                println!(
                    "Solution found! Challenge ID: {}, Nonce: {}, Hash: {}",
                    challenge.id, nonce, hash
                );
            }
        },
    }
}
