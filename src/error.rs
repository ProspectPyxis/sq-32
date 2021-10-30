use std::io;
use std::io::ErrorKind;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InputError {
    #[error("unexpected character (expected one of {expected:?}, found {found:?})")]
    UnexpectedCharMultiple { expected: Vec<char>, found: char },
    #[error("unexpected character (expected {expected:?}, found {found:?})")]
    UnexpectedCharSingle { expected: char, found: char },
}

impl InputError {
    pub fn to_io_error(self) -> io::Error {
        io::Error::new(ErrorKind::InvalidInput, self)
    }
}
