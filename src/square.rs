use crate::game::GameData;

pub mod directions {
    use super::Direction;

    pub const ALL: &[Direction] = &[
        Direction::NorthWest,
        Direction::NorthEast,
        Direction::SouthEast,
        Direction::SouthWest,
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    pub const CARDINALS: &[Direction] = &[
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    pub const DIAGONALS: &[Direction] = &[
        Direction::NorthWest,
        Direction::NorthEast,
        Direction::SouthEast,
        Direction::SouthWest,
    ];

    pub const NORTH_DIAGONALS: &[Direction] = &[Direction::NorthWest, Direction::NorthEast];

    pub const SOUTH_DIAGONALS: &[Direction] = &[Direction::SouthEast, Direction::SouthWest];
}

#[derive(PartialEq, Eq)]
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
    even_width: bool,
    bounding_area: usize,
}

impl Direction {
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
            even_width: dat.board_columns & 1 == 0,
            bounding_area: (dat.board_rows >> 1)
                * (dat
                    .board_columns
                    .saturating_add((dat.board_columns & 1) ^ 1)),
        }
    }
}

impl SquareCalc {
    pub const fn from_const(dat: GameData) -> SquareCalc {
        SquareCalc {
            width: dat.board_columns,
            even_width: dat.board_columns & 1 == 0,
            bounding_area: (dat.board_rows >> 1)
                * (dat
                    .board_columns
                    .saturating_add((dat.board_columns & 1) ^ 1)),
        }
    }

    #[inline]
    pub fn sparse(&self, x: usize) -> usize {
        x + (x * self.even_width as usize / self.width)
    }

    #[inline]
    pub fn dense(&self, x: usize) -> usize {
        if self.even_width {
            // Check if value would be out of bounds
            // assert!(self.is_bounded(x));
            // Integer ceiling division
            ((self.width * x) + self.width) / (self.width + 1)
        } else {
            x
        }
    }

    #[inline]
    pub fn dir_to_offset(&self, dir: &Direction) -> isize {
        let half_width = self.width as isize >> 1;
        match dir {
            Direction::NorthWest => -half_width - 1,
            Direction::NorthEast => -half_width,
            Direction::SouthEast => half_width + 1,
            Direction::SouthWest => half_width,
            _ => todo!(),
        }
    }

    #[inline]
    pub fn is_bounded(&self, x: usize) -> bool {
        x < self.bounding_area && (x % (self.width + 1) != self.width || !self.even_width)
    }

    // This function assumes an already sparsed number
    // It breaks automatically should the number attempt to go out of bounds
    pub fn add_dir(&self, x: usize, dir: &Direction, count: usize) -> usize {
        if count == 0 {
            return x;
        }

        let mut x = x as isize;

        for _ in 0..count {
            let new_val = x.checked_add(self.dir_to_offset(dir));
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

    pub fn add_dir_dense(&self, x: usize, dir: &Direction, count: usize) -> usize {
        self.dense(self.add_dir(self.sparse(x), dir, count))
    }

    #[inline]
    pub fn try_add_dir(&self, x: usize, dir: &Direction, count: usize) -> Option<usize> {
        let x = (x as isize).checked_add(self.dir_to_offset(dir).checked_mul(count as isize)?)?;
        if x.is_negative() || !self.is_bounded(x as usize) {
            None
        } else {
            Some(x as usize)
        }
    }

    #[inline]
    pub fn try_add_dir_dense(&self, x: usize, dir: &Direction, count: usize) -> Option<usize> {
        Some(self.dense(self.try_add_dir(self.sparse(x), dir, count)?))
    }
}
