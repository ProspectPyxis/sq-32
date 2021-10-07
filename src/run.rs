use crate::game::*;
use std::io;

pub fn run_cli() {
    let mut g = Game::new();

    println!("sq-32 0.1.0");
    loop {
        let mut cmd = String::new();
        if let Err(e) = io::stdin().read_line(&mut cmd) {
            eprintln!("command read failed: {:?}", e);
            continue;
        }

        if let Err(e) = parse_and_execute(&mut g, cmd.as_str()) {
            if e.as_str() == "exit" {
                return;
            } else {
                eprintln!("{}", e);
            }
        }
    }
}

pub fn parse_and_execute<'a>(g: &'a mut Game, cmd: &str) -> Result<&'a mut Game, String> {
    let split_cmd = cmd.split_whitespace().collect::<Vec<_>>();

    match split_cmd[0] {
        "init" => {
            g.init();
            g.print();
        }
        "fen" => {
            g.set_to_fen(&split_cmd[1])?;
            g.print();
        }
        "move" => {
            g.make_move(&split_cmd[1])?;
            g.print();
        }
        "print" => g.print(),
        "exit" => return Err("exit".to_string()),
        _ => return Err("invalid command".to_string()),
    };

    Ok(g)
}
