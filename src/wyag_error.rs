use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid Argument")]
    InvalidArgument(&'static str),
    #[error("I/O Error")]
    IoError(#[from] std::io::Error),
}