use sq_32::board::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut b = Board::new();

    b.set_to_fen(INITIAL_BOARD_FEN)?;
    b.validate();
    println!("{}", b.to_console_string());

    Ok(())
}
