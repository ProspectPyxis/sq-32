use crate::error::{InputError, MoveError};

pub enum Color {
    White,
    Black,
}

pub enum Rank {
    Man,
    King,
}

pub struct PieceStandard {
    pub color: Color,
    pub rank: Rank,
}

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

pub trait Game: Sized {
    type B: Bitboard;
    type M: Move;
    type P;

    fn game_data() -> GameData;

    fn make_move(&mut self, mv: Self::M) -> Result<Self, MoveError<Self::M>>;

    fn pos(&mut self, pos: &str) -> Result<Self, InputError>;
}

pub trait Bitboard: Game {
    fn set_piece_at(&mut self, piece: Option<Self::P>, pos: u8);
}

pub trait Move: Game {
    fn match_string(&self, movestr: &str) -> bool;

    fn to_string(&self) -> &str;
}
