use std::io::{self, Error, ErrorKind};

pub mod command;

pub fn invalid_argument(message: &'static str) -> io::Error {
    Error::new(ErrorKind::InvalidInput, message)
}

pub mod repository;