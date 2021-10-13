use crate::board;
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
    #[error("FEN validation error: {0}")]
    FenValidationError(FenValidationError),
    #[error("game-related error: {0}")]
    GameError(GameError),
    #[error("{0}")]
    Generic(String),
    #[error("unknown error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum FenValidationError {
    #[error("not a valid ascii string")]
    NotAscii,
    #[error("incorrect amount of fields (expected 5, got {0})")]
    IncorrectFieldCount(usize),
    #[error(
        "invalid first char for field {field} (expected one of {expected:?}, found {found:?})"
    )]
    InvalidFieldStart {
        field: usize,
        expected: Vec<char>,
        found: char,
    },
    #[error("fen string terminated too early")]
    TerminatedEarly,
    #[error("invalid field length (expected {expected}, found {found})")]
    InvalidFieldLength { expected: usize, found: usize },
    #[error("position is out of bounds (expected between 1 and {max}, got {pos})")]
    PosOutOfBounds { pos: usize, max: u8 },
    #[error("failed to parse int {intstring:?}: {error:?}")]
    ParseIntError {
        intstring: String,
        error: num::ParseIntError,
    },
    #[error("multiple pieces set on the same square at pos {0}")]
    InvalidSquare(usize),
}

#[derive(Error, Debug)]
pub enum GameError {
    #[error("move {mv:?} not found for player {player:?}")]
    MoveNotFound { mv: String, player: board::Player },
    #[error("multiple moves match {mv:?} (found {count})")]
    MoveAmbiguous { mv: String, count: usize },
    #[error("move start square ({0}) is empty")]
    MoveStartEmpty(u8),
    #[error("move target square ({0}) is occupied")]
    MoveTargetNotEmpty(u8),
    #[error("cannot unmake move ({}) on current position", .0.to_string(false))]
    UnmakeMoveFailed(board::Move),
    #[error("pos {0} is out of bounds")]
    PosOutOfBounds(u8),
    #[error("unknown error")]
    Unknown,
}
