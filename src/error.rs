use thiserror::Error;

#[derive(Error, Debug)]
pub enum RbkError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Connection timeout")]
    Timeout,

    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Write error: {0}")]
    WriteError(String),

    #[error("Client disposed")]
    Disposed,

    #[error("Bad API number: {0}")]
    BadApiNo(i32),

    #[error("No such robot")]
    NoSuchRobot,
}

pub type RbkResult<T> = Result<T, RbkError>;
