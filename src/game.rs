use crate::error::BoardError;
use std::str::FromStr;

pub mod default_piece {
    pub const WHITE_MAN: Piece = Piece {
        color: Color::White,
        rank: Rank::Man,
    };
    pub const WHITE_KING: Piece = Piece {
        color: Color::White,
        rank: Rank::King,
    };
    pub const BLACK_MAN: Piece = Piece {
        color: Color::Black,
        rank: Rank::Man,
    };
    pub const BLACK_KING: Piece = Piece {
        color: Color::Black,
        rank: Rank::King,
    };

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum Color {
        White,
        Black,
    }

    pub enum Rank {
        Man,
        King,
    }

    pub struct Piece {
        pub color: Color,
        pub rank: Rank,
    }

    impl Color {
        pub fn opposite(self) -> Color {
            if self == Color::White {
                Color::Black
            } else {
                Color::White
            }
        }
    }
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
    type M: Move;
    type UndoData;

    fn make_move(&mut self, mv: Self::M) -> Result<&Self, BoardError>;

    fn unmake_move(&mut self, mv: Self::M, undo: Self::UndoData) -> Result<&Self, BoardError>;
}

pub trait Bitboard: FromStr {
    type P;

    fn set_piece_at(&mut self, piece: Option<Self::P>, pos: u8) -> Result<&Self, BoardError>;

    fn get_piece_at(&self, pos: u8) -> Option<Self::P>;

    fn is_valid(&self) -> bool;
}

pub trait Move {
    fn match_string(&self, movestr: &str) -> bool {
        movestr == self.to_string(true).as_str()
    }

    fn to_string(&self, longform: bool) -> String;
}
