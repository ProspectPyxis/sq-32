use crate::error::{BoardError, InputError, Sq32Error};
use crate::game::default_piece::*;
use crate::game::{Bitboard, Game, GameData, GenMoves, Move};
use crate::square::{Direction, SquareCalc};
use dotbits::{BitManip, BitVec};
use std::str::FromStr;

const DATA_ENGLISH: GameData = GameData {
    id: "english",
    board_rows: 8,
    board_columns: 8,
};

const SCALC: SquareCalc = SquareCalc::from_const(DATA_ENGLISH);

pub struct GameEnglishDraughts {
    board: BBEnglishDraughts,
    active_player: Color,
}

#[derive(Default)]
pub struct BBEnglishDraughts {
    black: u32,
    white: u32,
    men: u32,
    kings: u32,
}

#[derive(Clone)]
pub struct MoveEnglishDraughts {
    pub from: usize,
    pub to: usize,
    pub captures: u32,
    in_between: Vec<usize>,
}

impl Game for GameEnglishDraughts {
    type M = MoveEnglishDraughts;
    // UndoData is a u32 representation of captured kings
    // It also contains info on whether this is a promotion
    type UndoData = u32;

    fn init() -> Self {
        GameEnglishDraughts {
            board: BBEnglishDraughts::from_str("bbbbbbbbbbbbeeeeeeeewwwwwwwwwwww")
                .expect("initial position failed"),
            active_player: Color::White,
        }
    }

    fn undo_data_of_move(&self, mv: &Self::M) -> Self::UndoData {
        let mut val = mv.captures & self.board.kings;
        let piece = self
            .board
            .get_piece_at(mv.from)
            .expect("move.from is empty - should never fire");

        let crownhead = match piece.color {
            Color::White => 0,
            Color::Black => 7,
        };

        if piece.rank == Rank::Man && mv.to / (DATA_ENGLISH.board_columns >> 1) == crownhead {
            val.bit_on(mv.to)
                .expect("move.to is too big - should never fire");
        }

        val
    }

    fn make_move(&mut self, mv: &Self::M) -> Result<&Self, BoardError> {
        let mut start_piece = if let Some(p) = self.board.get_piece_at(mv.from) {
            p
        } else {
            return Err(BoardError::UnexpectedEmpty(mv.from));
        };
        if self.board.get_piece_at(mv.to).is_some() {
            return Err(BoardError::UnexpectedNonEmpty(mv.to));
        }
        if mv.captures != 0 {
            if (self.board.white | self.board.black) & mv.captures != mv.captures {
                return Err(BoardError::UnexpectedEmpty(
                    *((self.board.white | self.board.black) & !(mv.captures))
                        .bits()
                        .ones()
                        .first()
                        .unwrap(),
                ));
            }
            for i in mv.captures.bits().ones() {
                self.board.set_piece_at(None, i)?;
            }
        }

        let crownhead = match start_piece.color {
            Color::White => 0,
            Color::Black => 7,
        };
        if mv.to / (DATA_ENGLISH.board_columns >> 1) == crownhead && start_piece.rank == Rank::Man {
            start_piece.rank = Rank::King;
        }

        self.board.set_piece_at(Some(start_piece), mv.to)?;
        self.board.set_piece_at(None, mv.from)?;

        self.active_player = self.active_player.opposite();

        Ok(self)
    }

    fn unmake_move(&mut self, mv: &Self::M, undo: Self::UndoData) -> Result<&Self, BoardError> {
        let mut end_piece = if let Some(p) = self.board.get_piece_at(mv.to) {
            p
        } else {
            return Err(BoardError::UnexpectedEmpty(mv.to));
        };

        if self.board.get_piece_at(mv.from).is_some() {
            return Err(BoardError::UnexpectedNonEmpty(mv.from));
        }

        if undo.bit_get(mv.to).unwrap() && end_piece.rank == Rank::King {
            end_piece.rank = Rank::Man;
        }

        if mv.captures != 0 {
            for i in mv.captures.bits().ones() {
                self.board.set_piece_at(
                    Some(Piece {
                        color: end_piece.color.opposite(),
                        rank: if undo.bit_get(i).unwrap() {
                            Rank::King
                        } else {
                            Rank::Man
                        },
                    }),
                    i,
                )?;
            }
        }

        self.board.set_piece_at(Some(end_piece), mv.from)?;
        self.board.set_piece_at(None, mv.to)?;

        self.active_player = self.active_player.opposite();

        Ok(self)
    }

    fn gen_moves(&mut self) -> Vec<Self::M> {
        let mut moves: Vec<Self::M> = Vec::new();

        let bitboard = match self.active_player {
            Color::White => self.board.white.bits().ones(),
            Color::Black => self.board.black.bits().ones(),
        };

        // First loop - get all captures
        for i in &bitboard {
            let mut caps = self.captures_at(*i);
            if !caps.is_empty() {
                moves.append(&mut caps);
            }
        }

        if !moves.is_empty() {
            return moves;
        }

        // Second loop - get all regular moves
        for i in &bitboard {
            let mut m = self.moves_at(*i);
            if !m.is_empty() {
                moves.append(&mut m);
            }
        }

        moves
    }
}

impl GenMoves for GameEnglishDraughts {
    fn valid_count() -> usize {
        DATA_ENGLISH.valid_squares_count()
    }

    fn moves_at(&self, pos: usize) -> Vec<Self::M> {
        let piece = match self.board.get_piece_at(pos) {
            Some(p) => p,
            None => return vec![],
        };

        let dirs = if piece.rank == Rank::King {
            Direction::diagonals()
        } else {
            match piece.color {
                Color::White => Direction::diagonals_north(),
                Color::Black => Direction::diagonals_south(),
            }
        };

        let mut moves: Vec<Self::M> = Vec::new();
        for d in dirs {
            let target_pos = if let Some(n) = SCALC.try_add_dir_dense(pos, &d, 1) {
                n
            } else {
                continue;
            };
            if self.board.get_piece_at(target_pos).is_none() {
                moves.push(MoveEnglishDraughts::new(pos, target_pos));
            }
        }

        moves
    }

    fn captures_at(&mut self, pos: usize) -> Vec<Self::M> {
        let piece = match self.board.get_piece_at(pos) {
            Some(p) => p,
            None => return vec![],
        };

        let dirs = if piece.rank == Rank::King {
            Direction::diagonals()
        } else {
            match piece.color {
                Color::White => Direction::diagonals_north(),
                Color::Black => Direction::diagonals_south(),
            }
        };
        let crownhead = match piece.color {
            Color::White => 0,
            Color::Black => 7,
        };

        let mut moves: Vec<Self::M> = Vec::new();

        for d in dirs {
            let neighbor_pos = if let Some(n) = SCALC.try_add_dir_dense(pos, &d, 1) {
                n
            } else {
                continue;
            };

            if let Some(p) = self.board.get_piece_at(neighbor_pos) {
                if piece.color == p.color {
                    continue;
                }
            } else {
                continue;
            }

            let target_pos = if let Some(n) = SCALC.try_add_dir_dense(pos, &d, 2) {
                n
            } else {
                continue;
            };
            if self.board.get_piece_at(target_pos).is_some() {
                continue;
            }

            let mut m = MoveEnglishDraughts::new(pos, target_pos);
            m.set_capture(neighbor_pos);

            if target_pos / (DATA_ENGLISH.board_columns / 2) == crownhead && piece.rank == Rank::Man
            {
                moves.push(m);
                continue;
            }

            let undo_data = self.undo_data_of_move(&m);

            self.make_move(&m)
                .expect("unexpected error when making move");
            let submoves = self.captures_at(target_pos);
            self.unmake_move(&m, undo_data)
                .expect("unexpected error when unmaking move");

            if submoves.is_empty() {
                moves.push(m);
                continue;
            }
            for mut submove in submoves {
                let mut new_move = MoveEnglishDraughts::new(pos, submove.to);
                new_move.in_between.append(&mut submove.in_between);
                new_move.in_between.push(m.to);
                new_move.merge_captures(m.captures);
                new_move.merge_captures(submove.captures);

                moves.push(new_move);
            }
        }

        moves
    }
}

impl FromStr for BBEnglishDraughts {
    type Err = Sq32Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != DATA_ENGLISH.valid_squares_count() as usize {
            return Err(InputError::InputLengthInvalid {
                expected: DATA_ENGLISH.valid_squares_count() as usize,
                len: s.len(),
            }
            .into());
        }

        let mut bb = Self::default();

        for (i, c) in s.chars().enumerate() {
            match c {
                'w' => {
                    bb.set_piece_at(Some(WHITE_MAN), i)?;
                }
                'W' => {
                    bb.set_piece_at(Some(WHITE_KING), i)?;
                }
                'b' => {
                    bb.set_piece_at(Some(BLACK_MAN), i)?;
                }
                'B' => {
                    bb.set_piece_at(Some(BLACK_KING), i)?;
                }
                'e' => (),
                _ => {
                    return Err(InputError::UnexpectedCharMultiple {
                        expected: vec!['w', 'W', 'b', 'B', 'e'],
                        found: c,
                    }
                    .into())
                }
            }
        }

        Ok(bb)
    }
}

impl Bitboard for BBEnglishDraughts {
    type P = Piece;

    fn set_piece_at(&mut self, piece: Option<Self::P>, pos: usize) -> Result<&Self, BoardError> {
        if pos > DATA_ENGLISH.valid_squares_count() - 1 {
            return Err(BoardError::PosOutOfBounds {
                max: DATA_ENGLISH.valid_squares_count() - 1,
                found: pos,
            });
        }

        self.black &= !(1 << pos);
        self.white &= !(1 << pos);
        self.men &= !(1 << pos);
        self.kings &= !(1 << pos);

        if let Some(p) = piece {
            match p.color {
                Color::Black => self.black |= 1 << pos,
                Color::White => self.white |= 1 << pos,
            }
            match p.rank {
                Rank::Man => self.men |= 1 << pos,
                Rank::King => self.kings |= 1 << pos,
            }
        }

        Ok(self)
    }

    fn get_piece_at(&self, pos: usize) -> Option<Self::P> {
        assert!(self.is_valid());
        assert!(pos < DATA_ENGLISH.valid_squares_count());

        if !(self.white | self.black).bit_get(pos).unwrap() {
            None
        } else {
            Some(Piece {
                color: if self.white.bit_get(pos).unwrap() {
                    Color::White
                } else {
                    Color::Black
                },
                rank: if self.men.bit_get(pos).unwrap() {
                    Rank::Man
                } else {
                    Rank::King
                },
            })
        }
    }

    fn is_valid(&self) -> bool {
        self.black & self.white == 0
            && self.men & self.kings == 0
            && (self.black | self.white) ^ (self.men | self.kings) == 0
    }
}

impl Move for MoveEnglishDraughts {
    fn to_string(&self, _: bool) -> String {
        let mut movestr = self.from.to_string();

        if self.captures == 0 {
            movestr.push('-');
        } else {
            if !self.in_between.is_empty() {
                for i in &self.in_between {
                    movestr.push('x');
                    movestr.push_str((i + 1).to_string().as_str());
                }
            }
            movestr.push('x');
        }
        movestr.push_str(self.to.to_string().as_str());

        movestr
    }
}

impl MoveEnglishDraughts {
    pub fn new(from: usize, to: usize) -> MoveEnglishDraughts {
        MoveEnglishDraughts {
            from,
            to,
            captures: 0,
            in_between: vec![],
        }
    }

    pub fn set_capture(&mut self, pos: usize) {
        self.captures.bit_on(pos).expect("out of bounds");
    }

    pub fn merge_captures(&mut self, captures: u32) {
        self.captures |= captures;
    }
}
