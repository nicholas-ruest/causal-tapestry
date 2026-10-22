//! Explicit errors crossing application boundaries.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("upstream integration failed: {0}")]
    Upstream(String),
    #[error("policy evaluation failed: {0}")]
    Policy(String),
    #[error("evidence persistence failed: {0}")]
    Evidence(String),
    #[error("I/O failed: {0}")]
    Io(String),
}

pub type Result<T> = std::result::Result<T, Error>;

