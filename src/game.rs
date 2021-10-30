use crate::error::InputError;
use std::io;
use std::str::FromStr;

#[derive(Default)]
pub struct Position {
    black: u64,
    white: u64,
    men: u64,
    kings: u64,
}

impl FromStr for Position {
    type Err = io::Error;

    fn from_str(pos_str: &str) -> Result<Self, Self::Err> {
        let mut pos = Position::default();

        for (i, c) in pos_str.chars().enumerate() {
            match c {
                'w' => {
                    pos.white |= 1 << i;
                    pos.men |= 1 << i;
                }
                'W' => {
                    pos.white |= 1 << i;
                    pos.kings |= 1 << i;
                }
                'b' => {
                    pos.black |= 1 << i;
                    pos.men |= 1 << i;
                }
                'B' => {
                    pos.black |= 1 << i;
                    pos.kings |= 1 << i;
                }
                'e' => (),
                _ => {
                    return Err(InputError::UnexpectedCharMultiple {
                        expected: vec!['w', 'W', 'b', 'B', 'e'],
                        found: c,
                    }
                    .to_io_error());
                }
            }
        }

        Ok(pos)
    }
}
