use crate::board::*;
use crate::utils;
use crate::utils::squares;

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
    utils::validate_fen("W:W21,22,23,24,25,26,27,28,29,30,31,32:B1,2,3,4,5,6,7,8,9,10,11,12:H0:F1")
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

#[test]
fn to_absolute() {
    assert_eq!(squares::to_absolute(0), 1);
    assert_eq!(squares::to_absolute(5), 10);
    assert_eq!(squares::to_absolute(18), 37);
    assert_eq!(squares::to_absolute(31), 62);
}

#[test]
fn from_absolute() {
    assert_eq!(squares::from_absolute(1).expect("error"), 0);
    assert_eq!(squares::from_absolute(10).expect("error"), 5);
    assert_eq!(squares::from_absolute(37).expect("error"), 18);
    assert_eq!(squares::from_absolute(62).expect("error"), 31);
    assert!(squares::from_absolute(6).is_none());
    assert!(squares::from_absolute(11).is_none());
}

#[test]
fn get_neighbors() {
    assert_eq!(
        squares::get_neighbor_at(9, squares::Dirs::NorthWest).expect("error"),
        5
    );
    assert_eq!(
        squares::get_neighbor_at(23, squares::Dirs::SouthEast).expect("error"),
        27
    );
}

#[test]
fn multiply_pos() {
    assert_eq!(squares::multiply_pos(12, squares::Dirs::SouthEast, 3), 25);
    assert_eq!(squares::multiply_pos(13, squares::Dirs::NorthEast, 8), 2);
}
