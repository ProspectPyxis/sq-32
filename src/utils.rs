pub mod squares {
    #[derive(Copy, Clone)]
    pub enum Dir {
        NorthWest = -9,
        NorthEast = -7,
        SouthEast = 9,
        SouthWest = 7,
    }

    impl Dir {
        pub fn as_vec() -> Vec<Dir> {
            vec![
                Dir::NorthWest,
                Dir::NorthEast,
                Dir::SouthEast,
                Dir::SouthWest,
            ]
        }
    }

    pub fn to_absolute(pos: u8) -> u8 {
        ((pos + 1) << 1) - ((pos >> 2 & 1) + 1)
    }

    pub fn from_absolute(abs: u8) -> Option<u8> {
        let offset = abs >> 3 & 1;
        if abs & 1 == offset {
            return None;
        }

        let new_pos = ((abs as i8 + offset as i8 + 1) >> 1) - 1;
        if new_pos < 0 {
            None
        } else {
            Some(new_pos as u8)
        }
    }

    pub fn get_neighbor_at(pos: u8, dir: Dir) -> Option<u8> {
        let new_abs = to_absolute(pos) as i8 + dir as i8;

        if !(0..=63).contains(&new_abs) {
            None
        } else {
            from_absolute(new_abs as u8)
        }
    }

    pub fn multiply_pos(pos: u8, dir: Dir, by: u8) -> u8 {
        let mut new_pos = pos;
        for _ in 0..by {
            match get_neighbor_at(new_pos, dir) {
                Some(num) => new_pos = num,
                None => break,
            }
        }
        new_pos
    }
}
