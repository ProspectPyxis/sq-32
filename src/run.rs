use crate::ai;
use crate::board::Player;
use crate::game::*;
use std::io;

#[derive(Debug, PartialEq, Eq)]
pub enum AutoGo {
    Off,
    White,
    Black,
    Both,
}

pub struct Config {
    pub print_after_commands: bool,
    pub auto_go: AutoGo,
}

pub struct Container {
    pub game: Game,
    pub config: Config,
}

impl AutoGo {
    pub fn player_matches(&self, player: Player) -> bool {
        match self {
            AutoGo::Both => true,
            AutoGo::White => player == Player::White,
            AutoGo::Black => player == Player::Black,
            AutoGo::Off => false,
        }
    }
}

impl Config {
    pub fn new() -> Config {
        Config {
            print_after_commands: true,
            auto_go: AutoGo::Off,
        }
    }
}

impl Container {
    pub fn new() -> Container {
        Container {
            game: Game::new(),
            config: Config::new(),
        }
    }

    pub fn parse_and_execute(
        &mut self,
        cmd: &str,
        is_console: bool,
    ) -> Result<&mut Container, String> {
        if cmd.trim().len() == 0 {
            return Ok(self);
        }

        let split_cmd = cmd.split_whitespace().collect::<Vec<_>>();

        let get_arg_at = |pos| {
            split_cmd
                .get(pos)
                .ok_or(format!("not enough arguments (required at least {})", pos))
        };

        let mut state_changed = false;

        match split_cmd[0] {
            "init" => {
                self.game.init();
                state_changed = true;
            }
            "fen" => match *get_arg_at(1)? {
                "get" => {
                    if !is_console {
                        return Err("cannot use this command if not in console mode".to_string());
                    }
                    println!("Current position FEN: {}", self.game.get_fen());
                }
                _ => {
                    self.game.set_to_fen(&get_arg_at(1)?)?;
                    state_changed = true;
                }
            },
            "pdn" => {
                if !is_console {
                    return Err("cannot use this command if not in console mode".to_string());
                }
                println!(
                    "Partial PDN for this game:\n{}",
                    self.game.get_partial_pdn()
                );
            }
            "move" => match *get_arg_at(1)? {
                "list" => {
                    if !is_console {
                        return Err("cannot use this command if not in console mode".to_string());
                    }
                    let movelist = self
                        .game
                        .board
                        .get_moves_for(self.game.current_player)
                        .iter()
                        .map(|x| x.to_string(true))
                        .collect::<Vec<_>>()
                        .join(", ");
                    println!("available moves:\n{}", movelist);
                }
                "undo" => {
                    let mut count = 1;
                    if let Ok(numstr) = get_arg_at(2) {
                        count = match numstr.parse::<usize>() {
                            Ok(num) => {
                                if num < 1 {
                                    1
                                } else {
                                    num
                                }
                            }
                            Err(e) => {
                                return Err(format!("undo count parse error: {:?}", e.kind()))
                            }
                        };
                    }
                    for _ in 0..count {
                        self.game.undo()?;
                    }
                    state_changed = true;
                }
                "hist" | "history" => {
                    let mut history = "Full move history:\n".to_string();
                    let mut pos = 1;

                    for m in &self.game.prev_moves {
                        history.push_str(format!("{}. {}\n", pos, m.m.to_string(false)).as_str());
                        pos += 1;
                    }

                    println!("{}", history);
                }
                _ => {
                    self.game.make_move(&get_arg_at(1)?)?;
                    state_changed = true;
                }
            },
            "go" => {
                if let Some(_) = self.game.winner {
                    return Err("cannot go because game is already over".to_string());
                }
                let mv = ai::go(&mut self.game).unwrap();
                self.game.make_move(mv.to_string(false).as_str())?;
                state_changed = true;
            }
            "print" => {
                if !is_console {
                    return Err("cannot print if not in console mode".to_string());
                } else {
                    self.game.print();
                }
            }
            "set" => match *get_arg_at(1)? {
                "print_after_commands" => {
                    self.config.print_after_commands =
                        get_arg_at(2)?.chars().next().unwrap_or('0') != '0';
                    println!(
                        "print_after_commands = {}",
                        self.config.print_after_commands
                    );
                }
                "auto_go" => {
                    match *get_arg_at(2)? {
                        "off" => self.config.auto_go = AutoGo::Off,
                        "white" => self.config.auto_go = AutoGo::White,
                        "black" => self.config.auto_go = AutoGo::Black,
                        "both" => self.config.auto_go = AutoGo::Both,
                        _ => return Err("invalid auto_go option (must be one of \"off\", \"white\", \"black\", \"both\")".to_string()),
                    };
                    println!("auto_go = {:?}", self.config.auto_go);
                }
                _ => return Err(format!("invalid config option {}", split_cmd[1])),
            },
            "exit" | "quit" | "q" => return Err("exit".to_string()),
            _ => return Err("invalid command".to_string()),
        };

        if state_changed {
            if self.config.auto_go.player_matches(self.game.current_player)
                && self.game.winner.is_none()
            {
                return self.parse_and_execute("go", is_console);
            } else if self.config.print_after_commands && is_console {
                self.game.print();
            }
        }

        Ok(self)
    }
}

pub fn run_cli() {
    let mut c = Container::new();

    println!("sq-32 0.1.0");
    loop {
        let mut cmd = String::new();
        if let Err(e) = io::stdin().read_line(&mut cmd) {
            eprintln!("command read failed: {:?}", e);
            continue;
        }
        cmd.make_ascii_lowercase();

        if let Err(e) = c.parse_and_execute(cmd.as_str(), true) {
            if e.as_str() == "exit" {
                return;
            } else {
                eprintln!("{}", e);
            }
        }
    }
}
