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

    #[derive(PartialEq, Eq)]
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
    pub board_rows: usize,
    pub board_columns: usize,
}

impl GameData {
    pub fn board_size(&self) -> usize {
        self.board_rows * self.board_columns
    }

    pub fn valid_squares_count(&self) -> usize {
        self.board_size() >> 1
    }
}

pub trait Game: Sized {
    type M: Move;
    type UndoData;

    fn make_move(&mut self, mv: &Self::M) -> Result<&Self, BoardError>;

    fn unmake_move(&mut self, mv: &Self::M, undo: Self::UndoData) -> Result<&Self, BoardError>;

    fn gen_moves(&mut self) -> Vec<Self::M>;
}

pub trait GenMoves: Game {
    fn valid_count() -> usize;

    fn moves_at(&self, pos: usize) -> Vec<Self::M>;

    fn captures_at(&mut self, pos: usize) -> Vec<Self::M>;

    fn all_moves(&self) -> Vec<Self::M> {
        let mut moves: Vec<Self::M> = Vec::new();
        for i in 0..Self::valid_count() {
            moves.append(&mut self.moves_at(i));
        }
        moves
    }

    fn all_captures(&mut self) -> Vec<Self::M> {
        let mut caps: Vec<Self::M> = Vec::new();
        for i in 0..Self::valid_count() {
            caps.append(&mut self.captures_at(i));
        }
        caps
    }
}

pub trait Bitboard: FromStr {
    type P;

    fn set_piece_at(&mut self, piece: Option<Self::P>, pos: usize) -> Result<&Self, BoardError>;

    fn get_piece_at(&self, pos: usize) -> Option<Self::P>;

    fn is_valid(&self) -> bool;
}

pub trait Move {
    fn match_string(&self, movestr: &str) -> bool {
        movestr == self.to_string(true).as_str()
    }

    fn to_string(&self, longform: bool) -> String;
}
