// TODO: Optimize this function, heavily
pub fn validate_fen<'a>(fen: &'a str) -> Result<&'a str, String> {
    if !fen.is_ascii() {
        return Err("provided FEN is not a valid ASCII string".to_string());
    }
    let uppercase_fen = fen.to_ascii_uppercase();

    let fen_fields = uppercase_fen.split(':').count();
    if fen_fields != 5 {
        return Err(format!(
            "invalid field count (expected 5, got {})",
            fen_fields
        ));
    }

    let split_fen: Vec<&str> = uppercase_fen.split(':').collect();
    if split_fen[0].len() != 1 {
        return Err("current player field has invalid length".to_string());
    }
    let current_player = split_fen[0].chars().next().unwrap_or('_');
    if current_player != 'W' && current_player != 'B' {
        return Err(format!(
            "invalid current player (expected W or B, got {})",
            current_player
        ));
    }

    let white_first_char = split_fen[1].chars().next().unwrap_or('_');
    if white_first_char != 'W' {
        return Err(format!(
            "invalid start of white pieces field (expected W, got {})",
            white_first_char
        ));
    }

    let white_pieces = split_fen[1][1..].split(',');
    let mut white_board: u32 = 0;

    for mut p in white_pieces {
        if p.chars().next().unwrap_or(' ') == 'K' {
            p = &p[1..];
        }
        let p = match p.parse::<u8>() {
            Ok(num) => num,
            Err(e) => return Err(format!("pos parse error: {:?}", e.kind())),
        };
        if p < 1 || p > 32 {
            return Err(format!(
                "white piece position is out of bounds (expected between 1 and 32, got {})",
                p
            ));
        }
        white_board |= 1 << (p - 1);
    }

    let black_first_char = split_fen[2].chars().next().unwrap_or('_');
    if black_first_char != 'B' {
        return Err(format!(
            "invalid start of black pieces field (expected B, got {})",
            black_first_char
        ));
    }

    let black_pieces = split_fen[2][1..].split(',');

    for mut p in black_pieces {
        if p.chars().next().unwrap_or(' ') == 'K' {
            p = &p[1..];
        }
        let p = match p.parse::<u8>() {
            Ok(num) => num,
            Err(e) => return Err(format!("pos parse error: {:?}", e.kind())),
        };
        if p < 1 || p > 32 {
            return Err(format!(
                "black piece position is out of bounds (expected between 1 and 32, got {})",
                p
            ));
        }
        if white_board & (1 << (p - 1)) != 0 {
            return Err(format!("pos {} has both a black and white piece", p));
        }
    }

    let mut halfmove_clock = split_fen[3].chars();
    let halfmove_first_char = halfmove_clock.next().unwrap_or('_');
    if halfmove_first_char != 'H' {
        return Err(format!(
            "invalid start of half move clock field (expected H, got {})",
            halfmove_first_char
        ));
    }
    if let Err(e) = halfmove_clock.as_str().parse::<u8>() {
        return Err(format!("half move clock parse error: {:?}", e));
    }

    let mut fullmove_number = split_fen[4].chars();
    let fullmove_first_char = fullmove_number.next().unwrap_or('_');
    if fullmove_first_char != 'F' {
        return Err(format!(
            "invalid start of half move clock field (expected F, got {})",
            fullmove_first_char
        ));
    }
    if let Err(e) = fullmove_number.as_str().parse::<u8>() {
        return Err(format!("full move clock parse error: {:?}", e));
    }

    Ok(fen)
}

pub mod squares {
    #[derive(Copy, Clone)]
    pub enum Dir {
        NorthWest = -9,
        NorthEast = -7,
        SouthEast = 9,
        SouthWest = 7,
    }

    impl Dir {
        pub fn iterator() -> impl Iterator<Item = Dir> {
            [
                Dir::NorthWest,
                Dir::NorthEast,
                Dir::SouthEast,
                Dir::SouthWest,
            ]
            .iter()
            .copied()
        }
    }

    pub fn to_absolute(pos: u8) -> u8 {
        (pos + 1 << 1) - ((pos >> 2 & 1) + 1)
    }

    pub fn from_absolute(abs: u8) -> Option<u8> {
        let offset = abs >> 3 & 1;
        if abs & 1 == offset {
            return None;
        }

        let new_pos = (abs as i8 + offset as i8 + 1 >> 1) - 1;
        if new_pos < 0 {
            None
        } else {
            Some(new_pos as u8)
        }
    }

    pub fn get_neighbor_at(pos: u8, dir: Dir) -> Option<u8> {
        let new_abs = to_absolute(pos) as i8 + dir as i8;

        if new_abs < 0 || new_abs > 63 {
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
