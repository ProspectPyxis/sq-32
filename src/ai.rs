use crate::board::*;
use crate::game::*;

pub fn go(g: &mut Game) -> Option<Move> {
    let moves = g.board.get_moves_for(g.current_player);
    if moves.is_empty() {
        return None;
    }
    let rand = if moves.len() > 1 {
        alea::u32_in_range(0, moves.len() as u32 - 1) as usize
    } else {
        0
    };

    moves.get(rand).cloned()
}
