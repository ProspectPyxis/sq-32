use crate::game::GameData;

pub struct SquareCalc {
    width: u8,
}

impl From<GameData> for SquareCalc {
    fn from(dat: GameData) -> Self {
        SquareCalc {
            width: dat.board_columns,
        }
    }
}

impl SquareCalc {
    pub fn sparse(&self, x: u8) -> u8 {
        if self.width & 1 == 1 {
            x
        } else {
            x + (x / self.width)
        }
    }

    pub fn dense(&self, x: u8) -> u8 {
        if self.width & 1 == 1 {
            x
        } else {
            // Check if value would be out of bounds
            assert_ne!(x % (self.width + 1), self.width);
            // Integer ceiling division
            ((self.width * x) + self.width) / (self.width + 1)
        }
    }
}
