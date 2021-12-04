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

#[derive(PartialEq, Eq, Copy, Clone)]
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

pub const fn precalculate_offsets(dat: GameData) -> [i32; 8] {
    let mut i: usize = 0;
    let mut arr = [0i32; 8];
    let half_width = dat.board_columns as i32 >> 1;
    loop {
        if i == 8 {
            break;
        }
        arr[i] = match i {
            0 => -half_width - 1,
            1 => -half_width,
            2 => half_width + 1,
            3 => half_width,
            4 => dat.board_columns as i32 + 1,
            5 => 1,
            6 => -(dat.board_columns as i32) - 1,
            7 => -1,
            _ => 0, // Should never reach this
        };
        i += 1;
    }
    arr
}
