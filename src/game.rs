use crate::board::*;
use crate::utils;
use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum DrawReason {
    Agreement,
    MoveLimit,
    ThreefoldRepetition,
}

#[derive(Debug, PartialEq)]
pub enum Winner {
    White,
    Black,
    Draw(DrawReason),
}

pub struct MoveWithHalfmove {
    pub m: Move,
    pub halfmove: u32,
}

pub struct Game {
    pub board: Board,
    pub current_player: Player,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
    pub prev_moves: Vec<MoveWithHalfmove>,
    pub winner: Option<Winner>,
}

impl fmt::Display for Winner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Winner::White => write!(f, "white won"),
            Winner::Black => write!(f, "black won"),
            Winner::Draw(reason) => match reason {
                DrawReason::Agreement => write!(f, "draw by agreement"),
                DrawReason::MoveLimit => write!(f, "draw by 40 move rule"),
                DrawReason::ThreefoldRepetition => write!(f, "draw by threefold repetition"),
            },
        }
    }
}

impl MoveWithHalfmove {
    pub fn new(m: Move, halfmove: u32) -> MoveWithHalfmove {
        MoveWithHalfmove { m, halfmove }
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            current_player: Player::White,
            halfmove_clock: 0,
            fullmove_number: 1,
            prev_moves: Vec::new(),
            winner: None,
        }
    }

    pub fn init(&mut self) -> &mut Game {
        self.board.set_initial();
        self.current_player = Player::White;
        self.halfmove_clock = 0;
        self.fullmove_number = 1;
        self.prev_moves = Vec::new();
        self.winner = None;

        self
    }

    pub fn get_console_string(&self) -> String {
        let mut full_string = self.board.to_console_string();

        full_string.push_str("\n\n");
        if let None = self.winner {
            full_string += &format!("{:?} to move\n", self.current_player);
        } else {
            full_string += &format!("Game over, {}\n", self.winner.as_ref().unwrap());
        }
        full_string += &format!(
            "Half moves = {}, Full moves = {}",
            self.halfmove_clock, self.fullmove_number
        );

        full_string
    }

    pub fn print(&self) {
        println!("{}", self.get_console_string());
    }

    pub fn set_to_fen(&mut self, fen: &str) -> Result<&mut Game, String> {
        let fen = utils::validate_fen(fen)?.to_ascii_uppercase();

        let split_fen = fen.split(':').collect::<Vec<_>>();
        self.current_player = match split_fen[0].chars().next().unwrap_or('_') {
            'W' => Player::White,
            'B' => Player::Black,
            _ => return Err("unexpected error".to_string()),
        };

        self.board.set_to_fen(&fen[..])?;

        self.halfmove_clock = split_fen[3][1..].parse::<u32>().unwrap();
        self.fullmove_number = split_fen[4][1..].parse::<u32>().unwrap();

        Ok(self)
    }

    pub fn make_move(&mut self, movestr: &str) -> Result<&mut Game, String> {
        if let Some(_) = self.winner {
            return Err("cannot make moves if game is already over".to_string());
        }

        let m = self.get_move_from_str(movestr)?;
        let prev_halfmove_clock = self.halfmove_clock;

        if m.captures.len() == 0
            && self.board.get_piece_at_pos(m.from).unwrap().p_type == PieceType::King
        {
            self.halfmove_clock += 1;
        } else {
            self.halfmove_clock = 0;
        }
        self.board.make_move(&m)?;
        self.current_player = match self.current_player {
            Player::White => Player::Black,
            Player::Black => {
                self.fullmove_number += 1;
                Player::White
            }
        };
        self.prev_moves
            .push(MoveWithHalfmove::new(m, prev_halfmove_clock));

        self.winner = self.check_winner();

        Ok(self)
    }

    pub fn undo(&mut self) -> Result<&mut Game, String> {
        if self.prev_moves.len() == 0 {
            return Ok(self);
        }

        let to_undo = self
            .prev_moves
            .pop()
            .ok_or("unexpected error".to_string())?;
        self.board.unmake_move(&to_undo.m)?;
        self.halfmove_clock = to_undo.halfmove;
        self.current_player = match self.current_player {
            Player::Black => Player::White,
            Player::White => {
                self.fullmove_number -= 1;
                Player::Black
            }
        };

        Ok(self)
    }

    pub fn get_move_from_str(&mut self, movestr: &str) -> Result<Move, String> {
        let mut moves = self.board.get_moves_for(self.current_player);
        moves.retain(|x| x.match_string(movestr));

        if moves.len() == 0 {
            Err(format!(
                "no move \"{}\" found for {:?}",
                movestr, self.current_player
            ))
        } else if moves.len() > 1 {
            Err(format!(
                "too many moves match move string (found {})",
                moves.len()
            ))
        } else {
            // Can I avoid using clone here?
            Ok(moves[0].clone())
        }
    }

    pub fn check_winner(&mut self) -> Option<Winner> {
        if self.board.get_moves_for(self.current_player).len() == 0 {
            return Some(match self.current_player {
                Player::White => Winner::Black,
                Player::Black => Winner::White,
            });
        }

        if self.halfmove_clock >= 80 {
            return Some(Winner::Draw(DrawReason::MoveLimit));
        }

        None
    }
}
