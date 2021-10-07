use crate::board::*;
use crate::utils;

pub enum DrawReason {
    Agreement,
    MoveLimit,
    ThreefoldRepetition,
}

pub enum Winner {
    White,
    Black,
    Draw(DrawReason),
}

pub struct Game {
    pub board: Board,
    pub current_player: Player,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
    pub winner: Option<Winner>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            current_player: Player::White,
            halfmove_clock: 0,
            fullmove_number: 1,
            winner: None,
        }
    }

    pub fn init(&mut self) -> &mut Game {
        self.board.set_initial();
        self.current_player = Player::White;
        self.halfmove_clock = 0;
        self.fullmove_number = 1;
        self.winner = None;

        self
    }

    pub fn get_console_string(&self) -> String {
        let mut full_string = self.board.to_console_string();

        full_string.push_str("\n\n");
        full_string.push_str(match self.current_player {
            Player::White => "White",
            Player::Black => "Black",
        });
        full_string.push_str(" to move\n");

        full_string += &format!(
            "Half moves = {}, Full moves = {}\n",
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
}
