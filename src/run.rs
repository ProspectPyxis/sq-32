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

        if let Err(e) = parse_and_execute(&mut c, cmd.as_str(), true) {
            if e.as_str() == "exit" {
                return;
            } else {
                eprintln!("{}", e);
            }
        }
    }
}

pub fn parse_and_execute<'a>(
    c: &'a mut Container,
    cmd: &str,
    is_console: bool,
) -> Result<&'a mut Container, String> {
    if cmd.trim().len() == 0 {
        return Ok(c);
    }

    let split_cmd = cmd.split_whitespace().collect::<Vec<_>>();

    let get_arg_at = |pos| {
        split_cmd
            .get(pos)
            .ok_or(format!("not enough arguments (required at least {})", pos))
    };

    match split_cmd[0] {
        "init" => {
            c.game.init();
            if c.config.print_after_commands && is_console {
                c.game.print();
            }
        }
        "fen" => {
            c.game.set_to_fen(&get_arg_at(1)?)?;
            if c.config.print_after_commands && is_console {
                c.game.print();
            }
        }
        "move" => {
            c.game.make_move(&get_arg_at(1)?)?;
            if c.config.print_after_commands && is_console {
                c.game.print();
            }
        }
        "print" => {
            if !is_console {
                return Err("cannot print if not in console mode".to_string());
            } else {
                c.game.print();
            }
        }
        "set" => match *get_arg_at(1)? {
            "print_after_commands" => {
                c.config.print_after_commands = get_arg_at(2)?.chars().next().unwrap_or('0') != '0';
            }
            _ => return Err(format!("invalid config option {}", split_cmd[1])),
        },
        "exit" => return Err("exit".to_string()),
        _ => return Err("invalid command".to_string()),
    };

    Ok(c)
}
