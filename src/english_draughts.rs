use crate::bit;
use crate::game::{Bitboard, Game, GameData, Move};

const ENGLISH_DRAUGHTS_DATA: GameData = GameData {
    id: "english",
    board_rows: 8,
    board_columns: 8,
};

pub struct GameEnglishDraughts {
    pub board: BBEnglishDraughts,
}

pub struct BBEnglishDraughts {
    black: u32,
    white: u32,
    men: u32,
    kings: u32,
}

pub struct MoveEnglishDraughts {
    from: u32,
    to: u32,
    captures: u32,
    in_between: u32,
}

impl Move for MoveEnglishDraughts {
    fn to_string(&self, _: bool) -> String {
        let mut movestr = bit::get_first_on_bit_pos(self.from)
            .expect("move.from is empty")
            .to_string();

        if self.captures == 0 {
            movestr.push('-');
        } else {
            let in_betweens = bit::get_all_on_bits(self.in_between);
            if !in_betweens.is_empty() {
                for i in in_betweens {
                    movestr.push('x');
                    movestr.push_str((i + 1).to_string().as_str());
                }
            }
            movestr.push('x');
        }

        movestr.push_str(
            bit::get_first_on_bit_pos(self.to)
                .expect("move.to is empty")
                .to_string()
                .as_str(),
        );

        movestr
    }
}
