use crate::game::Move;
use std::io;
use std::io::ErrorKind;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InputError {
    #[error("unexpected character (expected one of {expected:?}, found {found:?})")]
    UnexpectedCharMultiple { expected: Vec<char>, found: char },
    #[error("unexpected character (expected {expected:?}, found {found:?})")]
    UnexpectedCharSingle { expected: char, found: char },
    #[error("invalid input length (expected {expected}, got {len})")]
    InputLengthInvalid { expected: usize, len: usize },
}

#[derive(Error, Debug)]
pub enum MoveError<M: Move> {
    #[error("cannot make move ({}) on current position", .0.to_string(true))]
    MakeMoveFailed(M),
}

impl From<InputError> for io::Error {
    fn from(s: InputError) -> Self {
        io::Error::new(ErrorKind::InvalidInput, s)
    }
}
