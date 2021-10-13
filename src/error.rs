use std::num;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("input is empty")]
    EmptyInputError,
    #[error("invalid input: {0}")]
    InvalidInputError(String),
    #[error("bad data: {0}")]
    BadDataError(String),
    #[error("{0:?}")]
    ParseIntError(#[from] num::ParseIntError),
}
