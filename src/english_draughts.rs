use crate::error::{BoardError, InputError, Sq32Error};
use crate::game::default_piece::*;
use crate::game::{Bitboard, Game, GameData, Move};
use crate::square::SquareCalc;
use dotbits::{BitManip, BitVec};
use std::str::FromStr;

const DATA_ENGLISH: GameData = GameData {
    id: "english",
    board_rows: 8,
    board_columns: 8,
};

const SCALC: SquareCalc = SquareCalc::from_const(DATA_ENGLISH);

pub struct GameEnglishDraughts {
    board: BBEnglishDraughts,
    active_player: Color,
}

#[derive(Default)]
pub struct BBEnglishDraughts {
    black: u32,
    white: u32,
    men: u32,
    kings: u32,
}

pub struct MoveEnglishDraughts {
    from: u8,
    to: u8,
    captures: u32,
    in_between: Vec<u8>,
}

impl Game for GameEnglishDraughts {
    type M = MoveEnglishDraughts;
    // UndoData is a u32 representation of captured kings
    type UndoData = u32;

    fn make_move(&mut self, mv: Self::M) -> Result<&Self, BoardError> {
        let start_piece = self.board.get_piece_at(mv.from);
        if start_piece.is_none() {
            return Err(BoardError::UnexpectedEmpty(mv.from));
        }
        if self.board.get_piece_at(mv.to).is_some() {
            return Err(BoardError::UnexpectedNonEmpty(mv.to));
        }
        if mv.captures != 0 {
            if (self.board.white | self.board.black) & mv.captures != mv.captures {
                return Err(BoardError::UnexpectedEmpty(
                    *((self.board.white | self.board.black) & !(mv.captures))
                        .bits()
                        .ones()
                        .first()
                        .unwrap() as u8,
                ));
            }
            for i in mv.captures.bits().ones() {
                self.board.set_piece_at(None, i as u8)?;
            }
        }

        self.board.set_piece_at(start_piece, mv.to)?;
        self.board.set_piece_at(None, mv.from)?;

        self.active_player = self.active_player.opposite();

        Ok(self)
    }

    fn unmake_move(&mut self, mv: Self::M, undo: Self::UndoData) -> Result<&Self, BoardError> {
        let end_piece = self.board.get_piece_at(mv.to);
        if end_piece.is_none() {
            return Err(BoardError::UnexpectedEmpty(mv.to));
        }
        if self.board.get_piece_at(mv.to).is_some() {
            return Err(BoardError::UnexpectedNonEmpty(mv.from));
        }

        self.board.set_piece_at(end_piece, mv.from)?;
        self.board.set_piece_at(None, mv.to)?;

        if mv.captures != 0 {
            for i in mv.captures.bits().ones() {
                self.board.set_piece_at(
                    Some(Piece {
                        color: self.active_player,
                        rank: if undo.bit_get(i).unwrap() {
                            Rank::King
                        } else {
                            Rank::Man
                        },
                    }),
                    i as u8,
                )?;
            }
        }

        self.active_player = self.active_player.opposite();

        Ok(self)
    }

    fn gen_moves(&mut self) -> Vec<Self::M> {
        let mut moves: Vec<Self::M> = Vec::new();

        let bitboard = match self.active_player {
            Color::White => self.board.white,
            Color::Black => self.board.black,
        };

        moves
    }
}

impl FromStr for BBEnglishDraughts {
    type Err = Sq32Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != DATA_ENGLISH.valid_squares_count() as usize {
            return Err(InputError::InputLengthInvalid {
                expected: DATA_ENGLISH.valid_squares_count() as usize,
                len: s.len(),
            }
            .into());
        }

        let mut bb = Self::default();

        for (i, c) in s.chars().enumerate() {
            match c {
                'w' => {
                    bb.set_piece_at(Some(WHITE_MAN), i as u8)?;
                }
                'W' => {
                    bb.set_piece_at(Some(WHITE_KING), i as u8)?;
                }
                'b' => {
                    bb.set_piece_at(Some(BLACK_MAN), i as u8)?;
                }
                'B' => {
                    bb.set_piece_at(Some(BLACK_KING), i as u8)?;
                }
                'e' => (),
                _ => {
                    return Err(InputError::UnexpectedCharMultiple {
                        expected: vec!['w', 'W', 'b', 'B', 'e'],
                        found: c,
                    }
                    .into())
                }
            }
        }

        Ok(bb)
    }
}

impl Bitboard for BBEnglishDraughts {
    type P = Piece;

    fn set_piece_at(&mut self, piece: Option<Self::P>, pos: u8) -> Result<&Self, BoardError> {
        if pos > DATA_ENGLISH.valid_squares_count() - 1 {
            return Err(BoardError::PosOutOfBounds {
                max: DATA_ENGLISH.valid_squares_count() - 1,
                found: pos,
            });
        }

        self.black &= !(1 << pos);
        self.white &= !(1 << pos);
        self.men &= !(1 << pos);
        self.kings &= !(1 << pos);

        if let Some(p) = piece {
            match p.color {
                Color::Black => self.black |= 1 << pos,
                Color::White => self.white |= 1 << pos,
            }
            match p.rank {
                Rank::Man => self.men |= 1 << pos,
                Rank::King => self.kings |= 1 << pos,
            }
        }

        Ok(self)
    }

    fn get_piece_at(&self, pos: u8) -> Option<Self::P> {
        assert!(self.is_valid());
        let pos = pos as usize;

        if !(self.white | self.black).bit_get(pos).unwrap() {
            None
        } else {
            Some(Piece {
                color: if self.white.bit_get(pos).unwrap() {
                    Color::White
                } else {
                    Color::Black
                },
                rank: if self.men.bit_get(pos).unwrap() {
                    Rank::Man
                } else {
                    Rank::King
                },
            })
        }
    }

    fn is_valid(&self) -> bool {
        self.black & self.white == 0
            && self.men & self.kings == 0
            && (self.black | self.white) ^ (self.men | self.kings) == 0
    }
}

impl Move for MoveEnglishDraughts {
    fn to_string(&self, _: bool) -> String {
        let mut movestr = self.from.to_string();

        if self.captures == 0 {
            movestr.push('-');
        } else {
            if !self.in_between.is_empty() {
                for i in &self.in_between {
                    movestr.push('x');
                    movestr.push_str((i + 1).to_string().as_str());
                }
            }
            movestr.push('x');
        }
        movestr.push_str(self.to.to_string().as_str());

        movestr
    }
}
