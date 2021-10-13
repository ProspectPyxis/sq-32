use crate::error::Error;
use crate::game::*;
use crate::hub::*;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Worker {
    game: Game,
    binds: Vec<fn(&str)>,
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
            binds: Vec::new(),
        }
    }

    fn on_message(&self, response: &str) {
        for func in self.binds.iter() {
            func(response);
        }
    }

    pub fn default_bind(&mut self) {
        self.clear_binds();
        self.add_bind(|response| println!("{}", response));
    }

    pub fn add_bind(&mut self, bind: fn(&str)) {
        self.binds.push(bind);
    }

    pub fn clear_binds(&mut self) {
        self.binds = Vec::new();
    }

    pub fn send(&mut self, cmd: &str) -> Result<(), Error> {
        if cmd.trim().is_empty() {
            return Err(Error::EmptyInputError);
        }

        let mut scanner = Scanner::new(cmd);
        let command = scanner.get_key()?;

        match command.as_str() {
            "hub" => {
                self.on_message(format!("id name={} version={}", NAME, VERSION).as_str());
                self.on_message("wait");
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
                                return Err(Error::InvalidInputError("invalid fen".to_string()));
                                // TODO: Refactor this error handling
                            }
                        }
                        _ => {
                            return Err(Error::InvalidInputError(format!(
                                "invalid argument {}",
                                p.key
                            )));
                        }
                    }
                }
            }
            _ => {
                return Err(Error::InvalidInputError(format!(
                    "invalid command {}",
                    command
                )));
            }
        }

        Ok(())
    }
}
