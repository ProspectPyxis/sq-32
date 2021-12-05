use crate::error::{BoardError, InputError};
use crate::game::default_piece::*;
use crate::game::{Game, GameData};
use crate::square::{self, directions};
use arrayvec::ArrayVec;
use bit_iter::BitIter;
use dotbits::BitManip;
use std::fmt;
use std::str::FromStr;

const GAMEDATA: GameData = GameData {
    id: "english",
    board_rows: 8,
    board_columns: 8,
};

const OFFSETS: [i32; 8] = square::precalculate_offsets(GAMEDATA);

const CROWNHEAD: [u64; 2] = [0xF, 0x780000000];

// A mask to check for invalid squares
const GHOST: u64 = 0xFFFFFFF804020100;

const MAX_MOVES: usize = 32;

type MoveList = ArrayVec<MoveEnglishDraughts, MAX_MOVES>;

#[derive(Clone)]
pub struct GameEnglishDraughts {
    board: BBEnglishDraughts,
    active_player: Color,
}

#[derive(Default, Clone)]
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
}

impl Game for GameEnglishDraughts {
    type M = MoveEnglishDraughts;
    // UndoData is a u64 representation of captured kings
    // It also contains info on whether this is a promotion
    type UndoData = u64;
    type MoveList = MoveList;

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

        self.active_player = self.active_player.opposite();

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

        self.active_player = self.active_player.opposite();

        Ok(self)
    }

    fn gen_moves(&mut self) -> Self::MoveList {
        let mut moves: Self::MoveList = ArrayVec::new();

        self.board.gen_captures(&mut moves, self.active_player);

        if moves.is_empty() {
            self.board.gen_non_captures(&mut moves, self.active_player);
        }

        moves
    }
}

impl FromStr for BBEnglishDraughts {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != GAMEDATA.valid_squares_count() as usize {
            return Err(InputError::InputLengthInvalid {
                expected: GAMEDATA.valid_squares_count() as usize,
                len: s.len(),
            });
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
                    })
                }
            }
        }

        Ok(bb)
    }
}

impl BBEnglishDraughts {
    fn set_piece_at(&mut self, piece: Option<Piece>, pos: u64) {
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

    fn get_piece_at(&self, pos: u64) -> Option<Piece> {
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
    fn empty(&self) -> u64 {
        self.occupied() ^ !GHOST
    }

    #[inline]
    fn occupied(&self) -> u64 {
        self.black | self.white
    }

    pub fn is_valid(&self) -> bool {
        self.black & self.white == 0
            && !self.occupied() & self.kings == 0
            && self.occupied() & GHOST == 0
    }

    fn gen_non_captures(&self, movevec: &mut MoveList, turn: Color) {
        let (man_dirs, king_dirs) = match turn {
            Color::White => (directions::SOUTH_DIAGONALS, directions::NORTH_DIAGONALS),
            Color::Black => (directions::NORTH_DIAGONALS, directions::SOUTH_DIAGONALS),
        };
        let self_bb = self.current_board(&turn);

        macro_rules! get_in_dir {
            ($dirs:ident, $bb:ident) => {
                for d in $dirs {
                    let offset = OFFSETS[*d as usize];
                    let bb = self.empty().signed_left_shift(offset) & $bb;
                    for i in BitIter::from(bb) {
                        let pos = 1 << i;
                        movevec.push(MoveEnglishDraughts::new(
                            pos,
                            pos.signed_right_shift(offset),
                        ));
                    }
                }
            };
        }

        get_in_dir!(man_dirs, self_bb);

        let kings_bb = self_bb & self.kings;

        get_in_dir!(king_dirs, kings_bb);
    }

    fn gen_captures(&mut self, movevec: &mut MoveList, turn: Color) {
        let (man_dirs, king_dirs) = match turn {
            Color::White => (directions::SOUTH_DIAGONALS, directions::NORTH_DIAGONALS),
            Color::Black => (directions::NORTH_DIAGONALS, directions::SOUTH_DIAGONALS),
        };
        let self_bb = self.current_board(&turn);
        let opp_bb = self.opposite_board(&turn);

        macro_rules! get_in_dir {
            ($dirs:ident, $bb:ident) => {
                for d in $dirs {
                    let offset = OFFSETS[*d as usize];
                    let bb = (self.empty().signed_left_shift(offset) & opp_bb)
                        .signed_left_shift(offset)
                        & $bb;

                    for i in BitIter::from(bb) {
                        let pos = 1 << i;
                        let piece = match self.get_piece_at(pos) {
                            Some(p) => p,
                            None => return,
                        };

                        self.recursive_capture_part(
                            movevec,
                            &piece,
                            pos,
                            pos.signed_right_shift(offset * 2),
                            pos.signed_right_shift(offset),
                        );
                    }
                }
            };
        }

        get_in_dir!(man_dirs, self_bb);

        let king_bb = self_bb & self.kings;

        get_in_dir!(king_dirs, king_bb);
    }

    pub fn all_non_captures_for(&self, color: Color) -> u64 {
        // man_dirs and king_dirs should be opposite of normal, since this function works backwards
        // from empty squares
        let (self_bb, man_dirs, king_dirs) = match color {
            Color::White => (
                self.white,
                directions::SOUTH_DIAGONALS,
                directions::NORTH_DIAGONALS,
            ),
            Color::Black => (
                self.black,
                directions::NORTH_DIAGONALS,
                directions::SOUTH_DIAGONALS,
            ),
        };

        let mut movers: u64 = 0;

        for d in man_dirs {
            movers |= self.empty().signed_left_shift(OFFSETS[*d as usize]) & self_bb;
        }

        let kings_bb = self_bb & self.kings;
        if kings_bb != 0 {
            for d in king_dirs {
                movers |= self.empty().signed_left_shift(OFFSETS[*d as usize]) & kings_bb;
            }
        }

        movers
    }

    pub fn all_captures_for(&self, color: Color) -> u64 {
        // man_dirs and king_dirs should be opposite of normal, since this function works backwards
        // from empty squares
        let (self_bb, opposite_bb, man_dirs, king_dirs) = match color {
            Color::White => (
                self.white,
                self.black,
                directions::SOUTH_DIAGONALS,
                directions::NORTH_DIAGONALS,
            ),
            Color::Black => (
                self.black,
                self.white,
                directions::NORTH_DIAGONALS,
                directions::SOUTH_DIAGONALS,
            ),
        };

        let mut movers: u64 = 0;

        for d in man_dirs {
            let offset = OFFSETS[*d as usize];
            let shift = self.empty().signed_left_shift(offset) & opposite_bb;
            if shift != 0 {
                movers |= shift.signed_left_shift(offset) & self_bb;
            }
        }

        let kings_bb = self_bb & self.kings;
        if kings_bb != 0 {
            for d in king_dirs {
                let offset = OFFSETS[*d as usize];
                let shift = self.empty().signed_left_shift(offset) & opposite_bb;
                if shift != 0 {
                    movers |= shift.signed_left_shift(offset) & kings_bb;
                }
            }
        }

        movers
    }

    pub fn current_board(&self, color: &Color) -> u64 {
        match color {
            Color::White => self.white,
            Color::Black => self.black,
        }
    }

    pub fn opposite_board(&self, color: &Color) -> u64 {
        match color {
            Color::White => self.black,
            Color::Black => self.white,
        }
    }

    pub fn recursive_capture_part(
        &self,
        movevec: &mut MoveList,
        piece: &Piece,
        start: u64,
        pos: u64,
        captures: u64,
    ) {
        let dirs = if piece.rank == Rank::King {
            directions::DIAGONALS
        } else {
            match piece.color {
                Color::White => directions::NORTH_DIAGONALS,
                Color::Black => directions::SOUTH_DIAGONALS,
            }
        };
        let opp_bb = self.opposite_board(&piece.color) & !captures;
        let mut no_continues = true;

        for d in dirs {
            let offset = OFFSETS[*d as usize];
            if (pos.signed_left_shift(offset) & opp_bb).signed_left_shift(offset)
                & (self.empty() | start)
                != 0
            {
                no_continues = false;

                if piece.rank == Rank::Man && pos & CROWNHEAD[piece.color as usize] != 0 {
                    movevec.push(MoveEnglishDraughts::with_captures(
                        start,
                        pos.signed_left_shift(offset * 2),
                        captures | pos.signed_left_shift(offset),
                    ));
                    continue;
                }

                self.recursive_capture_part(
                    movevec,
                    piece,
                    start,
                    pos.signed_left_shift(offset * 2),
                    captures | pos.signed_left_shift(offset),
                );
            }
        }

        if no_continues {
            movevec.push(MoveEnglishDraughts::with_captures(start, pos, captures));
        }
    }
}

impl fmt::Display for MoveEnglishDraughts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let separator = if self.captures != 0 { 'x' } else { '-' };

        let densify = |x| x - (x / (GAMEDATA.board_columns as u32 + 1)) + 1;

        let from = densify(self.from.trailing_zeros());
        let to = densify(self.to.trailing_zeros());

        let mut captures = String::new();
        if self.captures != 0 {
            for i in BitIter::from(self.captures) {
                captures.push(separator);
                captures.push_str(&densify(i as u32).to_string());
            }
        }

        // This follows the hub protocol format
        // Instead of in-between squares we get the from and to, followed by all captures
        write!(f, "{}{}{}{}", from, separator, to, captures)
    }
}

impl MoveEnglishDraughts {
    pub fn new(from: u64, to: u64) -> MoveEnglishDraughts {
        MoveEnglishDraughts {
            from,
            to,
            captures: 0,
        }
    }

    pub fn with_captures(from: u64, to: u64, captures: u64) -> MoveEnglishDraughts {
        MoveEnglishDraughts { from, to, captures }
    }
}
