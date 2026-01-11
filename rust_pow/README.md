# Rust PoW 

A Proof of Work (PoW) system implementation in Rust using SHA-256 hashing. The server generates cryptographic challenges and the client solves them by finding nonces that produce hashes with a specific difficulty prefix.

## Quick Start

```bash
# Build
cargo build --release

# Create a challenge
cargo run -- server create

# Solve the challenge
cargo run -- client solve challenge.json

# Verify the solution
cargo run -- server verify solution.json
```

## Commands

**Server:**
- `cargo run -- server create` - Creates a new challenge in `challenge.json`
- `cargo run -- server verify <solution_file>` - Verifies a solution

**Client:**
- `cargo run -- client solve <challenge_file>` - Solves a challenge and writes to `solution.json`

## Code Coverage

Code coverage reports are generated using [cargo-tarpaulin](https://github.com/tarpaulin/tarpaulin). View the latest coverage report:

📊 **[View Coverage Report](./tarpaulin-report.html)**

## Dependencies

- `sha2` - SHA-256 hashing
- `rand` - Random number generation
- `serde` / `serde_json` - JSON serialization
- `thiserror` - Error handling
- `clap` - CLI parsing