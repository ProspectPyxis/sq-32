use crate::game::GameData;

pub enum Direction {
    NorthWest,
    NorthEast,
    SouthEast,
    SouthWest,
    North,
    East,
    South,
    West,
}

// This struct assists in calculating squares
// All boards with an even width have a "ghost column" attached to the end in-code
pub struct SquareCalc {
    width: u8,
}

impl Direction {
    pub fn all() -> Vec<Direction> {
        vec![
            Direction::NorthWest,
            Direction::NorthEast,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }

    pub fn ordinals() -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }

    pub fn diagonals() -> Vec<Direction> {
        vec![
            Direction::NorthWest,
            Direction::NorthEast,
            Direction::SouthEast,
            Direction::SouthWest,
        ]
    }

    pub fn diagonals_north() -> Vec<Direction> {
        vec![Direction::NorthWest, Direction::NorthEast]
    }

    pub fn diagonals_south() -> Vec<Direction> {
        vec![Direction::SouthEast, Direction::SouthWest]
    }

    pub fn opposite(self) -> Direction {
        match self {
            Direction::NorthWest => Direction::SouthEast,
            Direction::NorthEast => Direction::SouthWest,
            Direction::SouthEast => Direction::NorthWest,
            Direction::SouthWest => Direction::NorthEast,
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

impl From<GameData> for SquareCalc {
    fn from(dat: GameData) -> Self {
        SquareCalc {
            width: dat.board_columns,
        }
    }
}

impl SquareCalc {
    pub const fn from_const(dat: GameData) -> SquareCalc {
        SquareCalc {
            width: dat.board_columns,
        }
    }

    pub fn sparse(&self, x: u8) -> u8 {
        if self.width & 1 == 1 {
            x
        } else {
            x + (x / self.width)
        }
    }

    pub fn dense(&self, x: u8) -> u8 {
        if self.width & 1 == 1 {
            x
        } else {
            // Check if value would be out of bounds
            assert!(self.is_bounded(x));
            // Integer ceiling division
            ((self.width * x) + self.width) / (self.width + 1)
        }
    }

    pub fn is_bounded(&self, x: u8) -> bool {
        x % (self.width + 1) != self.width
    }

    // This function assumes an already sparsed number
    // It breaks automatically should the number attempt to go out of boundd
    pub fn add_dir(&self, x: u8, dir: Direction, count: u8) -> u8 {
        if count == 0 {
            return x;
        }

        let mut x = x as i8;
        let half_width = (self.width >> 1) as i8;
        let adder = match dir {
            Direction::NorthWest => -half_width - 1,
            Direction::NorthEast => -half_width,
            Direction::SouthEast => half_width + 1,
            Direction::SouthWest => half_width,
            _ => todo!(),
        };

        for i in 0..count {
            let new_val = x.checked_add(adder);
            if new_val.is_none() || new_val.unwrap() < 0 || !self.is_bounded(new_val.unwrap() as u8)
            {
                break;
            }
            x = new_val.unwrap();
        }

        x as u8
    }

    pub fn add_dir_dense(&self, x: u8, dir: Direction, count: u8) -> u8 {
        self.dense(self.add_dir(self.sparse(x), dir, count))
    }
}
