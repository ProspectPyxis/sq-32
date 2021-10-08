use crate::game::*;
use std::io;

pub struct Config {
    pub print_after_commands: bool,
}

pub struct Container {
    pub game: Game,
    pub config: Config,
}

impl Config {
    pub fn new() -> Config {
        Config {
            print_after_commands: true,
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

        match split_cmd[0] {
            "init" => {
                self.game.init();
                if self.config.print_after_commands && is_console {
                    self.game.print();
                }
            }
            "fen" => {
                self.game.set_to_fen(&get_arg_at(1)?)?;
                if self.config.print_after_commands && is_console {
                    self.game.print();
                }
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
                    if self.config.print_after_commands && is_console {
                        self.game.print();
                    }
                }
                _ => {
                    self.game.make_move(&get_arg_at(1)?)?;
                    if self.config.print_after_commands && is_console {
                        self.game.print();
                    }
                }
            },
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
                }
                _ => return Err(format!("invalid config option {}", split_cmd[1])),
            },
            "exit" => return Err("exit".to_string()),
            _ => return Err("invalid command".to_string()),
        };

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

        if let Err(e) = c.parse_and_execute(cmd.as_str(), true) {
            if e.as_str() == "exit" {
                return;
            } else {
                eprintln!("{}", e);
            }
        }
    }
}
