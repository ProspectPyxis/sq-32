mod board;

use board::*;

fn main() {
    let mut b = Board::init();

    b.set_piece(Some(WHITE_MAN), 20)
        .expect("Pos index out of bounds!");
    b.verify();
    println!("{}", b.to_console_string());
}
