pub mod english_draughts;
pub mod error;
pub mod game;
pub mod hub;
pub mod square;

use crate::game::Game;

pub fn perft(depth: usize, game: &mut english_draughts::GameEnglishDraughts) -> usize {
    if depth == 0 {
        return 1;
    }

    let mut nodes: usize = 0;

    let moves = game.gen_moves();
    for m in moves {
        let undo_data = game.undo_data_of_move(&m);
        game.make_move(&m).expect("fatal error");
        nodes += perft(depth - 1, game);
        game.unmake_move(&m, undo_data).expect("fatal error");
    }

    nodes
}
