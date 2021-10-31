use crate::error::MoveError;

pub struct GameData {
    pub id: &'static str,
    pub board_rows: u8,
    pub board_columns: u8,
}

impl GameData {
    pub fn board_size(&self) -> u8 {
        self.board_rows * self.board_columns
    }

    pub fn valid_squares_count(&self) -> u8 {
        self.board_size() >> 1
    }
}

pub trait Bitboard {
    type M: Move;

    fn make_move(&mut self, mv: Self::M) -> Result<Self, MoveError<Self::M>>
    where
        Self: Sized;
}

pub trait Move {
    fn match_string(&self, movestr: &str) -> bool;

    fn to_string(&self) -> &str;
}
