pub mod ai;
pub mod board;
pub mod game;
pub mod hub;
pub mod run;
pub mod utils;
pub mod worker;

use std::io;
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

#[cfg(test)]
mod tests;
