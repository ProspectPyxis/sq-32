use crate::bit;
use crate::error::InputError;
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
                'w' => {
                    bb.white |= 1 << i;
                    bb.men |= 1 << i;
                }
                'W' => {
                    bb.white |= 1 << i;
                    bb.kings |= 1 << i;
                }
                'b' => {
                    bb.black |= 1 << i;
                    bb.men |= 1 << i;
                }
                'B' => {
                    bb.black |= 1 << i;
                    bb.kings |= 1 << i;
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
