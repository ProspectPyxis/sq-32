pub mod board;

#[cfg(test)]
mod tests {
    use crate::board::*;

    #[test]
    fn set_board_one_piece() {
        let mut b = Board::new();
        b.set_piece(Some(WHITE_MAN), 0).expect("test failed");
        assert_eq!(b.white, 1);
        assert_eq!(b.black, 0);
        assert_eq!(b.men, 1);
        assert_eq!(b.kings, 0);
    }

    #[test]
    fn set_board_fen() {
        let mut b = Board::new();
        b.set_to_fen("W:W11,20,21,29:B2,3,4,5,10,18,K31:H0:F1")
            .expect("test failed");
        assert_eq!(b.white, 0b00010000000110000000010000000000);
        assert_eq!(b.black, 0b01000000000000100000001000011110);
        assert_eq!(b.men, 0b00010000000110100000011000011110);
        assert_eq!(b.kings, 0b01000000000000000000000000000000);
    }
}
