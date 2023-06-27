use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum MapError {
    #[error("Position out of bounds")]
    OutOfBounds,
}
