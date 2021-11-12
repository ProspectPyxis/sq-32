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
pub enum BoardError {
    #[error("pos out of bounds (expected maximum of {max}, found {found})")]
    PosOutOfBounds { max: u8, found: u8 },
    #[error("pos {0} is unexpectedly empty")]
    UnexpectedEmpty(u8),
    #[error("pos {0} is unexpectedly occupied")]
    UnexpectedNonEmpty(u8),
}

#[derive(Error, Debug)]
pub enum HubError {
    #[error("value is none")]
    NoValue,
    #[error("expected next token")]
    NoNextToken,
    #[error("unclosed quote mark")]
    UnclosedQuote,
}

#[derive(Error, Debug)]
pub enum Sq32Error {
    #[error("input error: {0}")]
    InputError(#[from] InputError),
    #[error("board error: {0}")]
    BoardError(#[from] BoardError),
    #[error("bit error: {0}")]
    BitError(#[from] dotbits::Error),
}
