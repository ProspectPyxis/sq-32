pub enum PieceColor {
    White,
    Black,
}

pub enum PieceType {
    Man,
    King,
}

pub struct Board {
    pub white: u32,
    pub black: u32,
    pub men: u32,
    pub kings: u32,
}

pub struct Piece {
    pub p_color: PieceColor,
    pub p_type: PieceType,
}

pub const WHITE_MAN: Piece = Piece {
    p_color: PieceColor::White,
    p_type: PieceType::Man,
};
pub const WHITE_KING: Piece = Piece {
    p_color: PieceColor::White,
    p_type: PieceType::King,
};
pub const BLACK_MAN: Piece = Piece {
    p_color: PieceColor::Black,
    p_type: PieceType::Man,
};
pub const BLACK_KING: Piece = Piece {
    p_color: PieceColor::Black,
    p_type: PieceType::King,
};

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
            full_string += &separator;
            full_string += &format!(
                "| {} |   | {} |   | {} |   | {} |   |\n",
                &chunk[4], &chunk[5], &chunk[6], &chunk[7]
            );
            full_string += &separator;
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

    pub fn set_piece(&mut self, piece: Option<Piece>, pos: u8) -> Result<(), &str> {
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
                    PieceColor::White => {
                        self.white |= 1 << pos;
                        self.black &= !(1 << pos);
                    }
                    PieceColor::Black => {
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
        Ok(())
    }

    fn get_piece_at_pos(&self, pos: u8) -> Option<Piece> {
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
}

fn get_piece_char(piece: Option<Piece>) -> char {
    match piece {
        None => ' ',
        Some(p) => match p.p_color {
            PieceColor::White => match p.p_type {
                PieceType::Man => 'M',
                PieceType::King => 'K',
            },
            PieceColor::Black => match p.p_type {
                PieceType::Man => 'm',
                PieceType::King => 'k',
            },
        },
    }
}
