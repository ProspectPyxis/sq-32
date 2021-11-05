use crate::bit;
use crate::error::InputError;
use crate::game::default_piece::*;
use crate::game::{Bitboard, Game, GameData, Move};
use std::io;
use std::str::FromStr;

const ENGLISH_DRAUGHTS_DATA: GameData = GameData {
    id: "english",
    board_rows: 8,
    board_columns: 8,
};

pub struct GameEnglishDraughts {
    pub board: BBEnglishDraughts,
}

#[derive(Default)]
pub struct BBEnglishDraughts {
    black: u32,
    white: u32,
    men: u32,
    kings: u32,
}

pub struct MoveEnglishDraughts {
    from: u32,
    to: u32,
    captures: u32,
    in_between: u32,
}

impl FromStr for BBEnglishDraughts {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != ENGLISH_DRAUGHTS_DATA.valid_squares_count() as usize {
            return Err(InputError::InputLengthInvalid {
                expected: ENGLISH_DRAUGHTS_DATA.valid_squares_count() as usize,
                len: s.len(),
            }
            .into());
        }

        let mut bb = Self::default();

        for (i, c) in s.chars().enumerate() {
            match c {
                'w' => bb.set_piece_at(Some(WHITE_MAN), i as u8),
                'W' => bb.set_piece_at(Some(WHITE_KING), i as u8),
                'b' => bb.set_piece_at(Some(BLACK_MAN), i as u8),
                'B' => bb.set_piece_at(Some(BLACK_KING), i as u8),
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

    fn set_piece_at(&mut self, piece: Option<Self::P>, pos: u8) {
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
    }

    fn get_piece_at(&self, pos: u8) -> Option<Self::P> {
        self.validate();

        if !bit::is_bit_on(self.white | self.black, pos) {
            None
        } else {
            Some(Piece {
                color: if bit::is_bit_on(self.white, pos) {
                    Color::White
                } else {
                    Color::Black
                },
                rank: if bit::is_bit_on(self.men, pos) {
                    Rank::Man
                } else {
                    Rank::King
                },
            })
        }
    }

    fn validate(&self) {
        assert_eq!(self.black & self.white, 0);
        assert_eq!(self.men & self.kings, 0);
        assert_eq!((self.black | self.white) ^ (self.men | self.kings), 0);
    }
}

impl Move for MoveEnglishDraughts {
    fn to_string(&self, _: bool) -> String {
        let mut movestr = bit::get_first_on_bit_pos(self.from)
            .expect("move.from is empty")
            .to_string();

        if self.captures == 0 {
            movestr.push('-');
        } else {
            let in_betweens = bit::get_all_on_bits(self.in_between);
            if !in_betweens.is_empty() {
                for i in in_betweens {
                    movestr.push('x');
                    movestr.push_str((i + 1).to_string().as_str());
                }
            }
            movestr.push('x');
        }

        movestr.push_str(
            bit::get_first_on_bit_pos(self.to)
                .expect("move.to is empty")
                .to_string()
                .as_str(),
        );

        movestr
    }
}
