use crate::error::FenValidationError;
use std::iter::Peekable;
use std::str::Chars;

// TODO: Optimize this function, heavily
pub fn validate_fen(fen: &str) -> Result<&str, FenValidationError> {
    if !fen.is_ascii() {
        return Err(FenValidationError::NotAscii);
    }

    let fen_fields = fen.split(':').count();
    if fen_fields != 5 {
        return Err(FenValidationError::IncorrectFieldCount(fen_fields));
    }

    let fen_iter = fen.to_ascii_uppercase();
    let mut fen_iter = fen.chars().peekable();

    let next_or_err =
        |iter: Peekable<Chars>| iter.next().ok_or(FenValidationError::TerminatedEarly);

    let match_first = |iter, field, expected: Vec<char>| {
        let found = next_or_err(iter)?;
        if !expected.contains(&found) {
            Err(FenValidationError::InvalidFieldStart {
                field,
                expected,
                found,
            })
        } else {
            Ok(())
        }
    };

    match_first(fen_iter, 0, vec!['W', 'B'])?;

    if next_or_err(fen_iter)? != ':' {
        return Err(FenValidationError::InvalidFieldLength {
            expected: 1,
            found: fen.split(':').next().expect("should never fail here").len(),
        });
    }

    let board: u32 = 0;

    let parse_or_err = |intstring: String| match intstring.parse::<usize>() {
        Ok(num) => Ok(num),
        Err(error) => Err(FenValidationError::ParseIntError { intstring, error }),
    };

    let check_piece_list = |b: u32, iter: Peekable<Chars>| {
        if iter.peek().is_some() && *iter.peek().unwrap() != ':' {
            let mut intstring = String::new();
            loop {
                let c = next_or_err(iter)?;
                if c == ',' || c == ':' {
                    let val = parse_or_err(intstring)?;

                    if !(1..=32).contains(&val) {
                        return Err(FenValidationError::PosOutOfBounds { pos: val, max: 32 });
                    }
                    if b & (1 << (val - 1)) != 0 {
                        return Err(FenValidationError::InvalidSquare(val));
                    }
                    b |= 1 << (val - 1);
                    intstring = String::new();

                    if c == ':' {
                        break;
                    }
                } else if c != 'K' && !intstring.is_empty() {
                    intstring.push(c);
                }
            }
        }

        Ok(())
    };

    match_first(fen_iter, 1, vec!['W'])?;
    check_piece_list(board, fen_iter)?;
    match_first(fen_iter, 2, vec!['B'])?;
    check_piece_list(board, fen_iter)?;

    let parse_clock = |iter: Peekable<Chars>| {
        let mut intstring = String::new();
        loop {
            let c = iter.next();
            if c.is_none() || c.unwrap() == ':' {
                parse_or_err(intstring)?;
                break;
            } else {
                intstring.push(c.unwrap());
            }
        }
        Ok(())
    };

    match_first(fen_iter, 3, vec!['H'])?;
    parse_clock(fen_iter)?;
    match_first(fen_iter, 4, vec!['F'])?;
    parse_clock(fen_iter)?;

    /*
    let split_fen: Vec<&str> = uppercase_fen.split(':').collect();
    if split_fen[0].len() != 1 {
        return Err(FenValidationError::InvalidFieldLength {
            expected: 1,
            found: split_fen[0].len(),
        });
    }
    let current_player = split_fen[0].chars().next().unwrap_or('_');
    if current_player != 'W' && current_player != 'B' {
        return Err(FenValidationError::InvalidFieldStart {
            field: 0,
            expected: "W or B".to_string(),
            found: current_player,
        });
    }

    let white_first_char = split_fen[1].chars().next().unwrap_or('_');
    if white_first_char != 'W' {
        return Err(FenValidationError::InvalidFieldStart {
            field: 1,
            expected: "W".to_string(),
            found: white_first_char,
        });
    }

    let white_pieces = split_fen[1][1..].split(',');
    let mut white_board: u32 = 0;

    if !split_fen[1][1..].is_empty() {
        for mut p in white_pieces {
            if p.chars().next().unwrap_or(' ') == 'K' {
                p = &p[1..];
            }
            let p = match p.parse::<u8>() {
                Ok(num) => num,
                Err(e) => {
                    return Err(FenValidationError::ParseIntError {
                        intstring: p.to_string(),
                        error: e,
                    })
                }
            };
            if !(1..=32).contains(&p) {
                return Err(FenValidationError::PosOutOfBounds { pos: p, max: 32 });
            }
            white_board |= 1 << (p - 1);
        }
    }

    let black_first_char = split_fen[2].chars().next().unwrap_or('_');
    if black_first_char != 'B' {
        return Err(FenValidationError::InvalidFieldStart {
            field: 2,
            expected: "B".to_string(),
            found: black_first_char,
        });
    }

    let black_pieces = split_fen[2][1..].split(',');

    if !split_fen[2][1..].is_empty() {
        for mut p in black_pieces {
            if p.chars().next().unwrap_or(' ') == 'K' {
                p = &p[1..];
            }
            let p = match p.parse::<u8>() {
                Ok(num) => num,
                Err(e) => {
                    return Err(FenValidationError::ParseIntError {
                        intstring: p.to_string(),
                        error: e,
                    })
                }
            };
            if !(1..=32).contains(&p) {
                return Err(FenValidationError::PosOutOfBounds { pos: p, max: 32 });
            }
            if white_board & (1 << (p - 1)) != 0 {
                return Err(FenValidationError::InvalidSquare(p));
            }
        }
    }

    let mut halfmove_clock = split_fen[3].chars();
    let halfmove_first_char = halfmove_clock.next().unwrap_or('_');
    if halfmove_first_char != 'H' {
        return Err(FenValidationError::InvalidFieldStart {
            field: 3,
            expected: "H".to_string(),
            found: halfmove_first_char,
        });
    }
    if let Err(e) = halfmove_clock.as_str().parse::<u8>() {
        return Err(FenValidationError::ParseIntError {
            intstring: halfmove_clock.as_str().to_string(),
            error: e,
        });
    }

    let mut fullmove_number = split_fen[4].chars();
    let fullmove_first_char = fullmove_number.next().unwrap_or('_');
    if fullmove_first_char != 'F' {
        return Err(FenValidationError::InvalidFieldStart {
            field: 4,
            expected: "F".to_string(),
            found: fullmove_first_char,
        });
    }
    if let Err(e) = fullmove_number.as_str().parse::<u8>() {
        return Err(FenValidationError::ParseIntError {
            intstring: fullmove_number.as_str().to_string(),
            error: e,
        });
    }
    */

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
