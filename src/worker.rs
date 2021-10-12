use crate::game::*;
use crate::hub::*;
use std::io::ErrorKind;

pub struct Worker {
    game: Game,
    pub on_message: fn(&str),
}

impl Default for Worker {
    fn default() -> Self {
        Self::new()
    }
}

impl Worker {
    pub fn new() -> Worker {
        Worker {
            game: Game::new(),
            on_message: |response| println!("{}", response),
        }
    }

    pub fn send(&mut self, cmd: &str) -> Result<(), ErrorKind> {
        if cmd.trim().is_empty() {
            return Err(ErrorKind::InvalidInput);
        }

        let mut scanner = Scanner::new(cmd);
        let command = scanner.get_key()?;

        match command.as_str() {
            "hub" => {
                (self.on_message)("id name=sq-32 version=0.3.0");
                (self.on_message)("wait");
            }
            "pos" => {
                while !scanner.is_done() {
                    let p = scanner.get_pair()?;
                    match p.key.as_str() {
                        "start" => {
                            self.game.init();
                        }
                        "fen" => {
                            if self.game.set_to_fen(p.val.as_str()).is_err() {
                                return Err(ErrorKind::InvalidInput); // TODO: Refactor this error handling
                            }
                        }
                        _ => return Err(ErrorKind::InvalidInput),
                    }
                }
            }
            _ => (),
        }

        Ok(())
    }
}
