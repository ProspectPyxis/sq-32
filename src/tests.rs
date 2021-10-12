use crate::board::*;
use crate::game::*;
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
        squares::get_neighbor_at(9, squares::Dir::NorthWest).expect("error"),
        5
    );
    assert_eq!(
        squares::get_neighbor_at(23, squares::Dir::SouthEast).expect("error"),
        27
    );
}

#[test]
fn multiply_pos() {
    assert_eq!(squares::multiply_pos(12, squares::Dir::SouthEast, 3), 25);
    assert_eq!(squares::multiply_pos(13, squares::Dir::NorthEast, 8), 2);
}

#[test]
fn get_moves() {
    let mut b = Board::new();
    b.set_to_fen("W:WK18:B:H0:F1").expect("unexpected error");

    let moves = b.get_piece_moves_at(17).expect("movelist is empty");
    assert_eq!(moves.len(), 4);
    assert_eq!(moves[1].to, 14);
}

#[test]
fn make_move() {
    let mut b = Board::new();
    b.set_to_fen("W:W18:B:H0:F1").expect("unexpected error");

    let m = Move::new(17, 14);
    b.make_move(&m).expect("unexpected error");
    assert_eq!(b.white, 1 << 14);
}

#[test]
fn get_captures() {
    let mut b = Board::new();
    b.set_to_fen("W:W22:B10,11,18:H0:F1")
        .expect("unexpected error");

    let moves = b.get_captures_from(21).expect("movelist is empty");
    assert_eq!(moves.len(), 2);
    assert_eq!(moves[0].in_between[0], 14);
    assert_eq!(moves[0].captures.len(), 2);
}

#[test]
fn make_capture() {
    let mut b = Board::new();
    b.set_to_fen("W:W22:B10,18:H0:F1")
        .expect("unexpected error");

    let mut m = Move::new(21, 5);
    let mut captures: Vec<Capture> = vec![
        Capture {
            piece: BLACK_MAN,
            pos: 17,
        },
        Capture {
            piece: BLACK_MAN,
            pos: 9,
        },
    ];
    m.captures.append(&mut captures);
    m.in_between.push(14);

    b.make_move(&m).expect("unexpected error");
    assert_eq!(b.black, 0);
    assert_eq!(b.white, 1 << 5);
}

#[test]
fn get_player_moves() {
    let mut b = Board::new();
    b.set_initial();

    let moves = b.get_moves_for(Player::White);
    assert_eq!(moves.len(), 7);
}

#[test]
fn get_player_captures_only() {
    let mut b = Board::new();
    b.set_to_fen("W:W8,19,22:B17,18:H0:F1")
        .expect("unexpected error");

    let moves = b.get_moves_for(Player::White);
    assert_eq!(moves.len(), 2);
}

#[test]
fn piece_promotion() {
    let mut b = Board::new();
    b.set_to_fen("W:W6:B:H0:F1").expect("unexpected error");

    let mut m = Move::new(5, 0);
    m.promote = true;

    b.make_move(&m).expect("unexpected error");
    assert_eq!(b.kings, 1);
}

#[test]
fn set_game_to_fen() {
    let mut g = Game::new();

    g.set_to_fen("B:W29,30,31,32:B1,2,3,4:H12:F8")
        .expect("validation failed");
    assert_eq!(g.current_player, Player::Black);
    assert_eq!(g.halfmove_clock, 12);
    assert_eq!(g.fullmove_number, 8);
}

#[test]
fn match_move_to_str() {
    let m = Move::new(0, 5);

    assert!(m.match_string("1-6"));
}

#[test]
fn match_capture_to_str() {
    let mut m = Move::new(0, 16);
    m.captures.push(Capture {
        piece: WHITE_MAN,
        pos: 5,
    });
    m.captures.push(Capture {
        piece: WHITE_MAN,
        pos: 13,
    });
    m.in_between.push(9);

    assert!(m.match_string("1x17"));
    assert!(m.match_string("1x10x17"));
    assert!(!m.match_string("1-17"));
}

#[test]
fn make_string_move() {
    let mut g = Game::new();
    g.set_to_fen("W:W18:B1:H0:F1").expect("unexpected error");

    g.make_move("18-15").expect("move failed");

    assert_eq!(g.board.white, 1 << 14);
}

#[test]
fn switch_players_after_move() {
    let mut g = Game::new();
    g.set_to_fen("W:W18:B1:H0:F1").expect("unexpected error");

    g.make_move("18-15").expect("move failed");

    assert_eq!(g.current_player, Player::Black);
}

#[test]
fn check_winner_after_move() {
    let mut g = Game::new();
    g.set_to_fen("W:W18:B15:H0:F1").expect("unexpected error");

    g.make_move("18x11").expect("move failed");

    assert_eq!(g.winner.unwrap(), Winner::White);
}
