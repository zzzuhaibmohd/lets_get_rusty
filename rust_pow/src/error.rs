use thiserror::Error;

#[derive(Error, Debug)]
pub enum PoWError {
    #[error("invalid solution")]
    InvalidSolution,

    #[error("challenge already solved")]
    AlreadySolved,
}
