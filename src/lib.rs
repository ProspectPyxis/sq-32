pub mod english_draughts;
pub mod error;
pub mod game;
pub mod hub;
pub mod square;

use crate::game::Game;

pub fn perft(depth: usize, game: &mut english_draughts::GameEnglishDraughts) -> (usize, usize) {
    if depth == 0 {
        return (1, 1);
    }

    let mut nodes: usize = 0;

    let moves = game.gen_moves();
    let mut mc = moves.len();
    if depth == 1 {
        return (mc, mc);
    }

    for m in moves {
        let undo_data = game.undo_data_of_move(&m);
        game.make_move(&m)
            .unwrap_or_else(|x| panic!("move error: {}\n{:#?}", x, m));
        let (node, count) = perft(depth - 1, game);
        game.unmake_move(&m, undo_data)
            .unwrap_or_else(|x| panic!("move error: {}\n{:#?}", x, m));
        nodes += node;
        mc = mc.max(count);
    }

    (nodes, mc)
}
