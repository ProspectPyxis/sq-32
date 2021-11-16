use crate::error::{BoardError, InputError, Sq32Error};
use crate::game::default_piece::*;
use crate::game::{Bitboard, Game, GameData, GenMoves, Move};
use crate::square::{self, directions};
use dotbits::BitManip;
use std::str::FromStr;

const GAMEDATA: GameData = GameData {
    id: "english",
    board_rows: 8,
    board_columns: 8,
};

const OFFSETS: [i32; 8] = square::gen_offsets(GAMEDATA);

const CROWNHEAD: [u64; 2] = [0xF, 0x780000000];

// A mask to check for invalid squares
const GHOST: u64 = 0xFFFFFFF804020100;

pub struct GameEnglishDraughts {
    board: BBEnglishDraughts,
    active_player: Color,
}

#[derive(Default)]
pub struct BBEnglishDraughts {
    black: u64,
    white: u64,
    kings: u64,
}

#[derive(Clone, Debug)]
pub struct MoveEnglishDraughts {
    pub from: u64,
    pub to: u64,
    pub captures: u64,
    in_between: Vec<u64>,
    is_final: bool,
}

impl Game for GameEnglishDraughts {
    type M = MoveEnglishDraughts;
    // UndoData is a u64 representation of captured kings
    // It also contains info on whether this is a promotion
    type UndoData = u64;

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

        if piece.rank == Rank::Man && mv.to & CROWNHEAD[piece.color as usize] != 0 {
            val |= mv.to;
        }

        val
    }

    fn make_move(&mut self, mv: &Self::M) -> Result<&Self, BoardError> {
        let mut start_piece = if let Some(p) = self.board.get_piece_at(mv.from) {
            p
        } else {
            return Err(BoardError::UnexpectedEmpty(
                mv.from.trailing_zeros() as usize
            ));
        };

        if mv.from != mv.to && self.board.occupied() & mv.to != 0 {
            return Err(BoardError::UnexpectedNonEmpty(
                mv.to.trailing_zeros() as usize
            ));
        }
        if mv.captures != 0 {
            /*
            if self.board.occupied() & mv.captures != mv.captures {
                return Err(BoardError::UnexpectedEmpty(
                    (self.board.occupied() & !(mv.captures)).trailing_zeros() as usize,
                ));
            }
            */
            self.board.black &= !mv.captures;
            self.board.white &= !mv.captures;
            self.board.kings &= !mv.captures;
        }

        if mv.to & CROWNHEAD[start_piece.color as usize] != 0 && start_piece.rank == Rank::Man {
            start_piece.rank = Rank::King;
        }

        self.board.set_piece_at(None, mv.from);
        self.board.set_piece_at(Some(start_piece), mv.to);

        if mv.is_final {
            self.active_player = self.active_player.opposite();
        }

        Ok(self)
    }

    fn unmake_move(&mut self, mv: &Self::M, undo: Self::UndoData) -> Result<&Self, BoardError> {
        let mut end_piece = if let Some(p) = self.board.get_piece_at(mv.to) {
            p
        } else {
            return Err(BoardError::UnexpectedEmpty(mv.to.trailing_zeros() as usize));
        };

        /*
        if self.board.get_piece_at(mv.from).is_some() {
            return Err(BoardError::UnexpectedNonEmpty(
                mv.from.trailing_zeros() as usize
            ));
        }
        */

        if undo & mv.to != 0 && end_piece.rank == Rank::King {
            end_piece.rank = Rank::Man;
        }

        if mv.captures != 0 {
            match end_piece.color {
                Color::White => self.board.black |= mv.captures,
                Color::Black => self.board.white |= mv.captures,
            };
            self.board.kings |= mv.captures & undo;
        }

        self.board.set_piece_at(None, mv.to);
        self.board.set_piece_at(Some(end_piece), mv.from);

        if mv.is_final {
            self.active_player = self.active_player.opposite();
        }

        Ok(self)
    }

    fn gen_moves(&mut self) -> Vec<Self::M> {
        let mut moves: Vec<Self::M> = Vec::with_capacity(16);

        let mut movers = self.any_captures();
        if movers != 0 {
            // First loop - get all captures
            while movers != 0 {
                let pos = 1 << movers.trailing_zeros();
                self.add_captures(pos, &mut moves);
                movers &= !pos;
            }
            return moves;
        }

        let mut movers = self.any_moves();
        if movers != 0 {
            // Second loop - get all regular moves
            while movers != 0 {
                let pos = 1 << movers.trailing_zeros();
                self.add_moves(pos, &mut moves);
                movers &= !pos;
            }
            return moves;
        }

        moves
    }
}

impl GenMoves for GameEnglishDraughts {
    type Bitsize = u64;

    fn add_moves(&self, pos: u64, movevec: &mut Vec<Self::M>) {
        let piece = match self.board.get_piece_at(pos) {
            Some(p) => p,
            None => return,
        };

        let dirs = if piece.rank == Rank::King {
            directions::DIAGONALS
        } else {
            match piece.color {
                Color::White => directions::NORTH_DIAGONALS,
                Color::Black => directions::SOUTH_DIAGONALS,
            }
        };

        for d in dirs {
            let target_pos = pos.signed_left_shift(OFFSETS[*d as usize]);
            if target_pos & self.board.empty() != 0 {
                movevec.push(MoveEnglishDraughts::new(pos, target_pos, true));
            }
        }
    }

    fn add_captures(&mut self, pos: u64, movevec: &mut Vec<Self::M>) {
        let piece = match self.board.get_piece_at(pos) {
            Some(p) => p,
            None => return,
        };

        let dirs = if piece.rank == Rank::King {
            directions::DIAGONALS
        } else {
            match piece.color {
                Color::White => directions::NORTH_DIAGONALS,
                Color::Black => directions::SOUTH_DIAGONALS,
            }
        };
        let opposite_bitboard = match piece.color {
            Color::White => self.board.black,
            Color::Black => self.board.white,
        };

        for d in dirs {
            let offset = OFFSETS[*d as usize];
            let neighbor_pos = pos.signed_left_shift(offset);

            if opposite_bitboard & neighbor_pos == 0 {
                // no piece to capture, or neighbor is out of bounds
                continue;
            }

            let target_pos = neighbor_pos.signed_left_shift(offset);
            if self.board.empty() & target_pos == 0 {
                // target is occupied, or target is out of bounds
                continue;
            }

            let mut m = MoveEnglishDraughts::new(pos, target_pos, false);
            m.merge_captures(neighbor_pos);

            if target_pos & CROWNHEAD[piece.color as usize] != 0 && piece.rank == Rank::Man {
                // move promotes - no need to continue
                m.is_final = true;
                movevec.push(m);
                continue;
            }

            let undo_data = self.undo_data_of_move(&m);
            self.make_move(&m)
                .expect("unexpected error when making move");

            if self.any_captures() & target_pos != 0 {
                let submoves = self.captures_at(target_pos);
                for submove in submoves {
                    let mut new_move = MoveEnglishDraughts::new(pos, submove.to, true);
                    new_move.in_between = submove.in_between;
                    new_move.in_between.push(m.to);
                    new_move.merge_captures(m.captures);
                    new_move.merge_captures(submove.captures);

                    movevec.push(new_move);
                }
            } else {
                self.unmake_move(&m, undo_data)
                    .expect("unexpected error when unmaking move");
                m.is_final = true;
                movevec.push(m);
                continue;
            }

            self.unmake_move(&m, undo_data)
                .expect("unexpected error when unmaking move");
        }
    }

    fn any_moves(&self) -> u64 {
        // man_dirs and king_dirs should be opposite of normal, since this function works backwards
        // from empty squares
        let (self_bb, man_dirs, king_dirs) = match self.active_player {
            Color::White => (
                self.board.white,
                directions::SOUTH_DIAGONALS,
                directions::NORTH_DIAGONALS,
            ),
            Color::Black => (
                self.board.black,
                directions::NORTH_DIAGONALS,
                directions::SOUTH_DIAGONALS,
            ),
        };

        let mut movers: u64 = 0;

        for d in man_dirs {
            movers |= self.board.empty().signed_left_shift(OFFSETS[*d as usize]) & self_bb;
        }

        let kings_bb = self_bb & self.board.kings;
        if kings_bb != 0 {
            for d in king_dirs {
                movers |= self.board.empty().signed_left_shift(OFFSETS[*d as usize]) & kings_bb;
            }
        }

        movers
    }

    fn any_captures(&self) -> u64 {
        // man_dirs and king_dirs should be opposite of normal, since this function works backwards
        // from empty squares
        let (self_bb, opposite_bb, man_dirs, king_dirs) = match self.active_player {
            Color::White => (
                self.board.white,
                self.board.black,
                directions::SOUTH_DIAGONALS,
                directions::NORTH_DIAGONALS,
            ),
            Color::Black => (
                self.board.black,
                self.board.white,
                directions::NORTH_DIAGONALS,
                directions::SOUTH_DIAGONALS,
            ),
        };

        let mut movers: u64 = 0;

        for d in man_dirs {
            let offset = OFFSETS[*d as usize];
            let shift = self.board.empty().signed_left_shift(offset) & opposite_bb;
            if shift != 0 {
                movers |= shift.signed_left_shift(offset) & self_bb;
            }
        }

        let kings_bb = self_bb & self.board.kings;
        if kings_bb != 0 {
            for d in king_dirs {
                let offset = OFFSETS[*d as usize];
                let shift = self.board.empty().signed_left_shift(offset) & opposite_bb;
                if shift != 0 {
                    movers |= shift.signed_left_shift(offset) & kings_bb;
                }
            }
        }

        movers
    }
}

impl FromStr for BBEnglishDraughts {
    type Err = Sq32Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != GAMEDATA.valid_squares_count() as usize {
            return Err(InputError::InputLengthInvalid {
                expected: GAMEDATA.valid_squares_count() as usize,
                len: s.len(),
            }
            .into());
        }

        let mut bb = Self::default();

        for (i, c) in s.chars().enumerate() {
            let i = i + (i / 8); // skip ghost squares
            match c {
                'w' => {
                    bb.set_piece_at(Some(WHITE_MAN), 1 << i);
                }
                'W' => {
                    bb.set_piece_at(Some(WHITE_KING), 1 << i);
                }
                'b' => {
                    bb.set_piece_at(Some(BLACK_MAN), 1 << i);
                }
                'B' => {
                    bb.set_piece_at(Some(BLACK_KING), 1 << i);
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
    type Bitsize = u64;

    fn set_piece_at(&mut self, piece: Option<Self::P>, pos: u64) {
        if let Some(p) = piece {
            match p.color {
                Color::Black => {
                    self.black |= pos;
                    self.white &= !pos;
                }
                Color::White => {
                    self.white |= pos;
                    self.black &= !pos;
                }
            }
            match p.rank {
                Rank::Man => {
                    self.kings &= !pos;
                }
                Rank::King => {
                    self.kings |= pos;
                }
            }
        } else {
            self.black &= !pos;
            self.white &= !pos;
            self.kings &= !pos;
        }
    }

    fn get_piece_at(&self, pos: u64) -> Option<Self::P> {
        // assert!(self.is_valid());
        if self.occupied() & pos == 0 {
            None
        } else {
            Some(Piece {
                color: if self.white & pos != 0 {
                    Color::White
                } else {
                    Color::Black
                },
                rank: if self.kings & pos != 0 {
                    Rank::King
                } else {
                    Rank::Man
                },
            })
        }
    }

    #[inline]
    fn empty(&self) -> Self::Bitsize {
        self.occupied() ^ !GHOST
    }

    #[inline]
    fn occupied(&self) -> Self::Bitsize {
        self.black | self.white
    }

    fn is_valid(&self) -> bool {
        self.black & self.white == 0
            && !self.occupied() & self.kings == 0
            && self.occupied() & GHOST == 0
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
    pub fn new(from: u64, to: u64, is_final: bool) -> MoveEnglishDraughts {
        MoveEnglishDraughts {
            from,
            to,
            captures: 0,
            in_between: vec![],
            is_final,
        }
    }

    pub fn merge_captures(&mut self, captures: u64) {
        self.captures |= captures;
    }
}
