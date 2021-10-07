pub mod board;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::board::*;
    use crate::utils;

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
    fn validate_fen() {
        utils::validate_fen(
            "W:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,11,12:H0:F1",
        )
        .expect("validation failed");
    }

    #[test]
    #[should_panic]
    fn invalid_fen() {
        utils::validate_fen("W:W21,,23:B1:H0:F1").expect("failed as expected");
    }

    #[test]
    fn set_board_fen() {
        let mut b = Board::new();
        b.set_to_fen("W:W11,20,21,29:B2,3,4,5,10,18,K31:H0:F1")
            .expect("validation failed");
        assert_eq!(b.white, 0b00010000000110000000010000000000);
        assert_eq!(b.black, 0b01000000000000100000001000011110);
        assert_eq!(b.men, 0b00010000000110100000011000011110);
        assert_eq!(b.kings, 0b01000000000000000000000000000000);
    }
}
