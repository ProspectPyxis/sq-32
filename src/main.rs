mod board;

use board::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut b = Board::new();

    b.set_piece(Some(WHITE_MAN), 20)?;
    b.verify();
    println!("{}", b.to_console_string());

    Ok(())
}
