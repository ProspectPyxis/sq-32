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
    width: usize,
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

    pub fn sparse(&self, x: usize) -> usize {
        if self.width & 1 == 1 {
            x
        } else {
            x + (x / self.width)
        }
    }

    pub fn dense(&self, x: usize) -> usize {
        if self.width & 1 == 1 {
            x
        } else {
            // Check if value would be out of bounds
            assert!(self.is_bounded(x));
            // Integer ceiling division
            ((self.width * x) + self.width) / (self.width + 1)
        }
    }

    pub fn is_bounded(&self, x: usize) -> bool {
        x % (self.width + 1) != self.width
    }

    // This function assumes an already sparsed number
    // It breaks automatically should the number attempt to go out of bounds
    pub fn add_dir(&self, x: usize, dir: Direction, count: usize) -> usize {
        if count == 0 {
            return x;
        }

        let mut x = x as isize;
        let half_width = (self.width >> 1) as isize;
        let adder = match dir {
            Direction::NorthWest => -half_width - 1,
            Direction::NorthEast => -half_width,
            Direction::SouthEast => half_width + 1,
            Direction::SouthWest => half_width,
            _ => todo!(),
        };

        for _ in 0..count {
            let new_val = x.checked_add(adder);
            if new_val.is_none()
                || new_val.unwrap() < 0
                || !self.is_bounded(new_val.unwrap() as usize)
            {
                break;
            }
            x = new_val.unwrap();
        }

        x as usize
    }

    pub fn add_dir_dense(&self, x: usize, dir: Direction, count: usize) -> usize {
        self.dense(self.add_dir(self.sparse(x), dir, count))
    }
}
