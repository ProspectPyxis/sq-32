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

    #[derive(PartialEq, Eq, Copy, Clone)]
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
    pub const fn board_size(&self) -> usize {
        self.board_rows * self.board_columns
    }

    pub const fn valid_squares_count(&self) -> usize {
        self.board_size() >> 1
    }
}

pub trait Game: Sized {
    type M: Move;
    type UndoData;

    fn init() -> Self;

    fn undo_data_of_move(&self, mv: &Self::M) -> Self::UndoData;

    fn make_move(&mut self, mv: &Self::M) -> Result<&Self, BoardError>;

    fn unmake_move(&mut self, mv: &Self::M, undo: Self::UndoData) -> Result<&Self, BoardError>;

    fn gen_moves(&mut self) -> Vec<Self::M>;

    fn increment_player(&mut self) {}
}

pub trait GenMoves: Bitboard {
    type M: Move;
    type Turn;

    fn gen_non_captures(&self, pos: Self::Bitsize, movevec: &mut Vec<Self::M>);

    fn gen_captures(&mut self, pos: Self::Bitsize, movevec: &mut Vec<Self::M>);

    fn any_moves_for(&self, turn: Self::Turn) -> Self::Bitsize;

    fn any_captures_for(&self, turn: Self::Turn) -> Self::Bitsize;

    #[inline]
    fn non_captures_at(&self, pos: Self::Bitsize) -> Vec<Self::M> {
        let mut v = Vec::new();
        self.gen_non_captures(pos, &mut v);
        v
    }

    #[inline]
    fn captures_at(&mut self, pos: Self::Bitsize) -> Vec<Self::M> {
        let mut v = Vec::new();
        self.gen_captures(pos, &mut v);
        v
    }
}

pub trait Bitboard: FromStr {
    type P;
    type Bitsize;

    fn set_piece_at(&mut self, piece: Option<Self::P>, pos: Self::Bitsize);

    fn get_piece_at(&self, pos: Self::Bitsize) -> Option<Self::P>;

    fn occupied(&self) -> Self::Bitsize;

    fn empty(&self) -> Self::Bitsize;

    fn is_valid(&self) -> bool;
}

pub trait Move {
    fn match_string(&self, movestr: &str) -> bool {
        movestr == self.to_string(true).as_str()
    }

    fn to_string(&self, longform: bool) -> String;
}
