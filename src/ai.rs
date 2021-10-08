use crate::game::*;

pub fn go<'a>(g: &'a mut Game) -> &'a mut Game {
    let moves = g.board.get_moves_for(g.current_player);
    if moves.len() == 0 {
        return g;
    }
    let rand = if moves.len() > 1 {
        alea::u32_in_range(0, moves.len() as u32 - 1) as usize
    } else {
        0
    };

    g.make_move(
        moves
            .get(rand)
            .expect("unexpected error")
            .to_string(false)
            .as_str(),
    )
    .expect("unexpected error");
    g
}
