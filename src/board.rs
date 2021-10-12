use crate::utils::squares;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Player {
    White,
    Black,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum PieceType {
    Man,
    King,
}

#[derive(Clone)]
pub struct Board {
    pub white: u32,
    pub black: u32,
    pub men: u32,
    pub kings: u32,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Piece {
    pub p_color: Player,
    pub p_type: PieceType,
}

#[derive(Clone)]
pub struct Capture {
    pub piece: Piece,
    pub pos: u8,
}

#[derive(Clone)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub in_between: Vec<u8>,
    pub captures: Vec<Capture>,
    pub promote: bool,
}

pub const WHITE_MAN: Piece = Piece {
    p_color: Player::White,
    p_type: PieceType::Man,
};
pub const WHITE_KING: Piece = Piece {
    p_color: Player::White,
    p_type: PieceType::King,
};
pub const BLACK_MAN: Piece = Piece {
    p_color: Player::Black,
    p_type: PieceType::Man,
};
pub const BLACK_KING: Piece = Piece {
    p_color: Player::Black,
    p_type: PieceType::King,
};

pub const INITIAL_BOARD_FEN: &str =
    "W:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,11,12:H0:F1";

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            white: 0,
            black: 0,
            men: 0,
            kings: 0,
        }
    }

    pub fn to_console_string(&self) -> String {
        let mut char_list = [' '; 32];

        for n in 0..32 {
            let c = self.get_piece_at_pos(n);
            char_list[n as usize] = get_piece_char(c);
        }

        let char_chunks = char_list.chunks(8);

        let separator = "+---+---+---+---+---+---+---+---+\n";

        let mut full_string = separator.to_string();

        let mut square_count: u8 = 4;

        for chunk in char_chunks {
            full_string += &format!(
                "|   | {} |   | {} |   | {} |   | {} | {}\n",
                &chunk[0], &chunk[1], &chunk[2], &chunk[3], square_count
            );
            full_string += separator;
            full_string += &format!(
                "| {} |   | {} |   | {} |   | {} |   |\n",
                &chunk[4], &chunk[5], &chunk[6], &chunk[7]
            );
            full_string += separator;
            square_count += 8;
        }
        full_string.push_str("  29      30      31      32");

        full_string
    }

    pub fn validate(&self) {
        if self.white & self.black != 0 {
            panic!("Invalid board: at least one square is both white and black at once. (ref: {:#034b})", self.white & self.black);
        }
        if self.men & self.kings != 0 {
            panic!("Invalid board: at least one square is both a man and a king at once. (ref: {:#034b})", self.men & self.kings);
        }
        let full_board: u32 = (self.white | self.black) ^ (self.men | self.kings);
        if full_board != 0 {
            panic!("Invalid board: piece color bitboards and piece type bitboards do not match. (ref: {:#034b})", full_board);
        }
    }

    pub fn set_piece(&mut self, piece: Option<Piece>, pos: u8) -> Result<&mut Board, &str> {
        if pos > 31 {
            return Err("piece index out of bounds");
        }
        match piece {
            None => {
                self.white &= !(1 << pos);
                self.black &= !(1 << pos);
                self.men &= !(1 << pos);
                self.kings &= !(1 << pos);
            }
            Some(p) => {
                match p.p_color {
                    Player::White => {
                        self.white |= 1 << pos;
                        self.black &= !(1 << pos);
                    }
                    Player::Black => {
                        self.white &= !(1 << pos);
                        self.black |= 1 << pos;
                    }
                }
                match p.p_type {
                    PieceType::Man => {
                        self.men |= 1 << pos;
                        self.kings &= !(1 << pos);
                    }
                    PieceType::King => {
                        self.men &= !(1 << pos);
                        self.kings |= 1 << pos;
                    }
                }
            }
        }
        Ok(self)
    }

    pub fn set_to_fen(&mut self, fen: &str) -> Result<&mut Board, String> {
        // Clear the board
        *self = Board::new();
        let split_fen: Vec<&str> = fen.split(':').collect();

        self.set_piece_by_fen_field(split_fen[1])?;
        self.set_piece_by_fen_field(split_fen[2])?;

        Ok(self)
    }

    pub fn set_piece_by_fen_field(&mut self, fen_field: &str) -> Result<(), String> {
        let col = match fen_field.chars().next().unwrap_or(' ') {
            'W' => Player::White,
            'B' => Player::Black,
            _ => return Err("invalid start of string".to_string()),
        };
        let fen_field = &fen_field[1..];

        if fen_field.is_empty() {
            return Ok(());
        }

        for mut p in fen_field.split(',') {
            let mut piece_type = PieceType::Man;
            if p.starts_with('K') {
                p = &p[1..];
                piece_type = PieceType::King;
            }
            let pos = p.parse::<u8>().unwrap() - 1;
            self.set_piece(
                Some(Piece {
                    p_color: col,
                    p_type: piece_type,
                }),
                pos,
            )?;
        }

        Ok(())
    }

    pub fn set_initial(&mut self) -> &mut Board {
        self.set_to_fen(INITIAL_BOARD_FEN)
            .expect("unexpected error")
    }

    pub fn get_piece_at_pos(&self, pos: u8) -> Option<Piece> {
        if pos > 31 {
            return None;
        }
        let matcher: u32 = 1 << pos;
        if (self.white & self.men) & matcher != 0 {
            Some(WHITE_MAN)
        } else if (self.black & self.men) & matcher != 0 {
            Some(BLACK_MAN)
        } else if (self.white & self.kings) & matcher != 0 {
            Some(WHITE_KING)
        } else if (self.black & self.kings) & matcher != 0 {
            Some(BLACK_KING)
        } else {
            None
        }
    }

    pub fn make_move(&mut self, m: &Move) -> Result<(), String> {
        if self.get_piece_at_pos(m.to).is_some() {
            return Err("target square is occupied".to_string());
        }
        let mut piece = self.get_piece_at_pos(m.from).unwrap();
        if m.promote {
            piece.p_type = PieceType::King;
        }
        self.set_piece(None, m.from)?;
        self.set_piece(Some(piece), m.to)?;
        if !m.captures.is_empty() {
            for p in &m.captures {
                self.set_piece(None, p.pos)?;
            }
        }
        Ok(())
    }

    pub fn unmake_move(&mut self, m: &Move) -> Result<(), String> {
        if self.get_piece_at_pos(m.to).is_none() {
            return Err("invalid move to unmake".to_string());
        }
        let mut piece = self.get_piece_at_pos(m.to).unwrap();
        if m.promote {
            piece.p_type = PieceType::Man;
        }
        self.set_piece(None, m.to)?;
        self.set_piece(Some(piece), m.from)?;
        if !m.captures.is_empty() {
            for p in &m.captures {
                self.set_piece(Some(p.piece), p.pos)?;
            }
        }
        Ok(())
    }

    pub fn get_moves_for(&mut self, player: Player) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        let bitboard = match player {
            Player::White => self.white,
            Player::Black => self.black,
        };

        // First loop - get captures only
        for n in 0..32 {
            if bitboard & 1 << n == 0 {
                continue;
            }
            if let Some(mut m) = self.get_captures_from(n) {
                if m.is_empty() {
                    continue;
                }
                moves.append(&mut m);
            }
        }

        // Second loop - if no captures available, get non-captures
        if moves.is_empty() {
            for n in 0..32 {
                if bitboard & 1 << n == 0 {
                    continue;
                }
                if let Some(mut m) = self.get_piece_moves_at(n) {
                    if m.is_empty() {
                        continue;
                    }
                    moves.append(&mut m);
                }
            }
        }

        moves
    }

    pub fn get_piece_moves_at(&self, pos: u8) -> Option<Vec<Move>> {
        let mut moves: Vec<Move> = Vec::new();

        let piece = match self.get_piece_at_pos(pos) {
            Some(p) => p,
            None => return None,
        };

        let mut dirs = squares::Dir::as_vec();
        if let PieceType::Man = piece.p_type {
            match piece.p_color {
                Player::White => dirs.retain(|&x| (x as i8) < 0),
                Player::Black => dirs.retain(|&x| (x as i8) > 0),
            }
        }

        let crownhead: u8 = match piece.p_color {
            Player::White => 0,
            Player::Black => 7,
        };

        for dir in dirs {
            let neighbor = match squares::get_neighbor_at(pos, dir) {
                Some(num) => num,
                None => continue,
            };
            if self.get_piece_at_pos(neighbor).is_none() {
                let mut m = Move::new(pos, neighbor);
                if piece.p_type == PieceType::Man && neighbor >> 2 == crownhead {
                    m.promote = true;
                }
                moves.push(m);
            }
        }

        Some(moves)
    }

    pub fn get_captures_from(&mut self, pos: u8) -> Option<Vec<Move>> {
        let mut moves: Vec<Move> = Vec::new();

        let piece = match self.get_piece_at_pos(pos) {
            Some(p) => p,
            None => return None,
        };

        let mut dirs = squares::Dir::as_vec();
        if let PieceType::Man = piece.p_type {
            match piece.p_color {
                Player::White => dirs.retain(|&x| (x as i8) < 0),
                Player::Black => dirs.retain(|&x| (x as i8) > 0),
            }
        }

        let crownhead: u8 = match piece.p_color {
            Player::White => 0,
            Player::Black => 7,
        };

        for dir in dirs {
            let neighbor = match squares::get_neighbor_at(pos, dir) {
                Some(num) => num,
                None => continue,
            };
            let target = match self.get_piece_at_pos(neighbor) {
                Some(p) => p,
                None => continue,
            };
            if target.p_color == piece.p_color {
                continue;
            }

            let square_to = squares::multiply_pos(pos, dir, 2);
            if self.get_piece_at_pos(square_to).is_some() {
                continue;
            }

            let mut m = Move::new(pos, square_to);
            m.captures.push(Capture {
                piece: target,
                pos: neighbor,
            });
            if square_to >> 2 == crownhead {
                m.promote = true;
                moves.push(m);
                continue;
            }
            self.make_move(&m)
                .expect("unexpected error when making move");
            let submoves = self.get_captures_from(square_to);
            self.unmake_move(&m)
                .expect("unexpected error when unmaking move");
            let submoves = match submoves {
                Some(v) => v,
                None => {
                    moves.push(m);
                    continue;
                }
            };
            // Can I avoid cloning here or is this the right use?
            for mut submove in submoves {
                let mut new_move = m.clone();
                new_move.in_between.push(m.to);
                new_move.in_between.append(&mut submove.in_between);
                new_move.captures.append(&mut submove.captures);
                new_move.to = submove.to;
                new_move.promote = submove.promote;
                moves.push(new_move);
            }
        }

        if !moves.is_empty() {
            Some(moves)
        } else {
            None
        }
    }
}

impl Move {
    pub fn new(from: u8, to: u8) -> Move {
        Move {
            from,
            to,
            in_between: Vec::new(),
            captures: Vec::new(),
            promote: false,
        }
    }

    pub fn match_string(&self, movestr: &str) -> bool {
        if movestr
            .chars()
            .any(|x| !x.is_numeric() && x != '-' && x != 'x')
        {
            return false;
        }

        // Move string cannot be both a capture and a normal move at once
        if movestr.chars().any(|x| x == '-') && movestr.chars().any(|x| x == 'x') {
            return false;
        }

        let separator = if movestr.chars().any(|x| x == '-') {
            '-'
        } else {
            'x'
        };

        let mut iterator = movestr.split(separator);

        let from = match iterator.next().unwrap().parse::<u8>() {
            Ok(num) => num - 1,
            Err(_) => return false,
        };
        if from != self.from {
            return false;
        }
        let to = match iterator.next_back().unwrap().parse::<u8>() {
            Ok(num) => num - 1,
            Err(_) => return false,
        };
        if to != self.to {
            return false;
        }

        if !self.captures.is_empty() && separator != 'x' {
            return false;
        }
        if self.captures.is_empty() && separator != '-' {
            return false;
        }

        for m in iterator {
            let in_between = match m.parse::<u8>() {
                Ok(num) => num - 1,
                Err(_) => return false,
            };

            if !self.in_between.contains(&in_between) {
                return false;
            }
        }

        true
    }

    pub fn to_string(&self, bracket_capture: bool) -> String {
        if self.captures.is_empty() {
            format!("{}-{}", self.from + 1, self.to + 1)
        } else {
            let mut movestr = format!("{}", self.from + 1);
            if !self.in_between.is_empty() {
                if bracket_capture {
                    movestr.push('(');
                }
                for i in &self.in_between {
                    movestr.push_str(format!("x{}", i + 1).as_str());
                }
                if bracket_capture {
                    movestr.push(')');
                }
            }
            movestr.push_str(format!("x{}", self.to + 1).as_str());

            movestr
        }
    }
}

fn get_piece_char(piece: Option<Piece>) -> char {
    match piece {
        None => ' ',
        Some(p) => match p {
            WHITE_MAN => 'M',
            BLACK_MAN => 'm',
            WHITE_KING => 'K',
            BLACK_KING => 'k',
        },
    }
}
