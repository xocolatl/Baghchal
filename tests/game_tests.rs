use baghchal::{Board, Piece, Winner};

#[test]
fn test_initial_board() {
    let board = Board::new();
    assert_eq!(board.cells[0], Piece::Tiger); // Top-left
    assert_eq!(board.cells[4], Piece::Tiger); // Top-right
    assert_eq!(board.cells[20], Piece::Tiger); // Bottom-left
    assert_eq!(board.cells[24], Piece::Tiger); // Bottom-right
    assert_eq!(board.goats_in_hand, 20);
    assert_eq!(board.captured_goats, 0);
}

#[test]
fn test_place_goat() {
    let mut board = Board::new();

    // Valid placement
    assert!(board.place_goat(12));
    assert_eq!(board.cells[12], Piece::Goat);
    assert_eq!(board.goats_in_hand, 19);

    // Invalid placements
    assert!(!board.place_goat(12)); // Already occupied
    assert!(!board.place_goat(0)); // Tiger's position
    assert!(!board.place_goat(25)); // Out of bounds
}

#[test]
fn test_tiger_basic_moves() {
    let mut board = Board::new();

    // Valid moves
    assert!(board.move_tiger(0, 1)); // Right
    assert!(board.move_tiger(1, 0)); // Left
    assert!(board.move_tiger(0, 5)); // Down
    assert!(board.move_tiger(5, 0)); // Up

    // Invalid moves
    assert!(!board.move_tiger(12, 13)); // No tiger at source
    assert!(!board.move_tiger(0, 25)); // Out of bounds
    assert!(!board.move_tiger(0, 7)); // Too far
}

#[test]
fn test_tiger_diagonal_moves() {
    let mut board = Board::new();

    // Valid diagonal moves from corner
    assert!(board.move_tiger(0, 6)); // Diagonal from top-left

    // Reset tiger position
    board.cells[6] = Piece::Empty;
    board.cells[0] = Piece::Tiger;

    // Invalid diagonal moves
    assert!(!board.move_tiger(1, 7)); // Not a diagonal position
    assert!(!board.move_tiger(0, 8)); // Too far
}

#[test]
fn test_tiger_captures() {
    let mut board = Board::new();

    // Setup: place a goat and test capture
    board.place_goat(1);
    assert!(board.move_tiger(0, 2)); // Jump over goat
    assert_eq!(board.captured_goats, 1);
    assert_eq!(board.cells[1], Piece::Empty); // Goat should be captured

    // Setup diagonal capture
    board.cells[2] = Piece::Empty;
    board.cells[0] = Piece::Tiger;
    board.place_goat(6);
    assert!(board.move_tiger(0, 12)); // Diagonal jump
    assert_eq!(board.captured_goats, 2);
    assert_eq!(board.cells[6], Piece::Empty); // Goat should be captured

    // Invalid captures
    board.cells[12] = Piece::Empty;
    board.cells[0] = Piece::Tiger;
    assert!(!board.move_tiger(0, 2)); // No goat to capture

    board.place_goat(1);
    board.place_goat(2);
    assert!(!board.move_tiger(0, 2)); // Destination occupied
}

#[test]
fn test_game_over() {
    let mut board = Board::new();

    // Game not over initially
    assert!(!board.is_game_over());

    // Capture 5 goats
    board.captured_goats = 5;
    assert!(board.is_game_over());

    // Reset and test with no goats in hand
    board.captured_goats = 4;
    board.goats_in_hand = 0;
    assert!(!board.is_game_over());

    // Capture one more goat
    board.captured_goats = 5;
    assert!(board.is_game_over());
}

#[test]
fn test_diagonal_positions() {
    let board = Board::new();

    // Test valid diagonal positions
    assert!(board.is_diagonal_allowed(0));
    assert!(board.is_diagonal_allowed(2));
    assert!(board.is_diagonal_allowed(4));
    assert!(board.is_diagonal_allowed(10));
    assert!(board.is_diagonal_allowed(12));
    assert!(board.is_diagonal_allowed(14));
    assert!(board.is_diagonal_allowed(20));
    assert!(board.is_diagonal_allowed(22));
    assert!(board.is_diagonal_allowed(24));

    // Test invalid diagonal positions
    assert!(!board.is_diagonal_allowed(1));
    assert!(!board.is_diagonal_allowed(3));
    assert!(!board.is_diagonal_allowed(5));
    assert!(!board.is_diagonal_allowed(11));
    assert!(!board.is_diagonal_allowed(13));
}

#[test]
fn test_diagonal_positions_and_moves() {
    let mut board = Board::new();

    // Test diagonal moves from corner (0)
    assert!(board.move_tiger(0, 6)); // Down-right diagonal

    // Reset board
    board = Board::new();
    assert!(board.move_tiger(4, 8)); // Down-left diagonal

    // Test diagonal moves from middle positions
    board = Board::new();
    board.cells[12] = Piece::Tiger; // Place tiger in center
    board.cells[0] = Piece::Empty; // Remove tiger from corner

    // All valid diagonal moves from center
    assert!(board.move_tiger(12, 6)); // Up-left
    board.cells[6] = Piece::Empty;
    board.cells[12] = Piece::Tiger;

    assert!(board.move_tiger(12, 8)); // Up-right
    board.cells[8] = Piece::Empty;
    board.cells[12] = Piece::Tiger;

    assert!(board.move_tiger(12, 16)); // Down-left
    board.cells[16] = Piece::Empty;
    board.cells[12] = Piece::Tiger;

    assert!(board.move_tiger(12, 18)); // Down-right
}

#[test]
fn test_diagonal_captures() {
    let mut board = Board::new();

    // Test diagonal capture from top-left corner
    board.place_goat(6); // Place goat in diagonal position
    assert!(board.move_tiger(0, 12)); // Should capture diagonally
    assert_eq!(board.captured_goats, 1);
    assert_eq!(board.cells[6], Piece::Empty); // Goat should be captured

    // Test diagonal capture from center
    board = Board::new();
    board.cells[12] = Piece::Tiger; // Place tiger in center
    board.cells[4] = Piece::Empty; // Remove tiger from corner
    board.place_goat(8); // Place goat for capture
    assert!(board.move_tiger(12, 4)); // Should capture diagonally up-right
    assert_eq!(board.captured_goats, 1);
    assert_eq!(board.cells[8], Piece::Empty);

    // Test invalid diagonal captures
    board = Board::new();
    board.place_goat(7); // Place goat in non-diagonal position
    assert!(!board.move_tiger(0, 14)); // Should not allow diagonal capture through non-diagonal position
}

#[test]
fn test_invalid_diagonal_moves() {
    let mut board = Board::new();

    // Test moves from non-diagonal positions
    board.cells[1] = Piece::Tiger; // Place tiger in non-diagonal position
    board.cells[0] = Piece::Empty; // Remove tiger from corner

    // Attempt invalid diagonal moves
    assert!(!board.move_tiger(1, 7)); // Should not allow diagonal move
    assert!(!board.move_tiger(1, 5)); // Should still allow orthogonal move

    // Test invalid diagonal destination
    board = Board::new();
    assert!(!board.move_tiger(0, 7)); // Cannot move to non-diagonal position diagonally
}

#[test]
fn test_goat_basic_moves() {
    let mut board = Board::new();

    // Place a goat
    board.place_goat(12); // Center position

    // Test orthogonal moves
    assert!(board.move_goat(12, 11)); // Left
    assert!(board.move_goat(11, 12)); // Right
    assert!(board.move_goat(12, 7)); // Up
    assert!(board.move_goat(7, 12)); // Down

    // Test invalid moves
    assert!(!board.move_goat(12, 14)); // Too far
    assert!(!board.move_goat(12, 0)); // To occupied position (tiger)
    assert!(!board.move_goat(0, 1)); // Moving from tiger position
}

#[test]
fn test_goat_diagonal_moves() {
    let mut board = Board::new();

    // Place a goat at center (diagonal position)
    board.place_goat(12);

    // Test valid diagonal moves
    assert!(board.move_goat(12, 6)); // Up-left
    board.cells[12] = Piece::Goat; // Reset
    board.cells[6] = Piece::Empty;

    assert!(board.move_goat(12, 8)); // Up-right
    board.cells[12] = Piece::Goat; // Reset
    board.cells[8] = Piece::Empty;

    assert!(board.move_goat(12, 16)); // Down-left
    board.cells[12] = Piece::Goat; // Reset
    board.cells[16] = Piece::Empty;

    assert!(board.move_goat(12, 18)); // Down-right

    // Test invalid diagonal moves
    board = Board::new();
    board.place_goat(7); // Non-diagonal position
    assert!(!board.move_goat(7, 13)); // Cannot move diagonally from non-diagonal position
}

#[test]
fn test_tiger_win() {
    let mut board = Board::new();

    // Capture 5 goats
    for _ in 0..5 {
        board.place_goat(1);
        assert!(board.move_tiger(0, 2)); // Capture goat at position 1
        board.cells[2] = Piece::Empty;
        board.cells[0] = Piece::Tiger;
    }

    assert_eq!(board.get_winner(), Winner::Tigers);
    assert!(board.is_game_over());
}

#[test]
fn test_goat_win() {
    let mut board = Board::new();

    // Place all tigers in the top row
    board.cells[0] = Piece::Tiger;
    board.cells[1] = Piece::Tiger;
    board.cells[2] = Piece::Tiger;
    board.cells[3] = Piece::Tiger;

    // Surround them with goats
    board.cells[4] = Piece::Goat;
    board.cells[5] = Piece::Goat;
    board.cells[6] = Piece::Goat;
    board.cells[7] = Piece::Goat;
    board.cells[8] = Piece::Goat;
    board.cells[9] = Piece::Goat;
    board.cells[10] = Piece::Goat;
    board.cells[11] = Piece::Goat;
    board.cells[12] = Piece::Goat;
    board.cells[13] = Piece::Goat;
    board.cells[14] = Piece::Goat;

    // Remove tigers from the bottom row
    board.cells[20] = Piece::Empty;
    board.cells[24] = Piece::Empty;

    assert_eq!(board.get_winner(), Winner::Goats);
    assert!(board.is_game_over());
}

#[test]
fn test_game_not_over() {
    let mut board = Board::new();

    // Place some goats but don't trap tigers
    board.place_goat(12);
    board.place_goat(7);
    board.place_goat(11);

    assert_eq!(board.get_winner(), Winner::None);
    assert!(!board.is_game_over());
}

#[test]
fn test_trapped_tigers_but_enough_captures() {
    let mut board = Board::new();

    // Capture 5 goats
    for _ in 0..5 {
        board.place_goat(1);
        assert!(board.move_tiger(0, 2));
        board.cells[2] = Piece::Empty;
        board.cells[0] = Piece::Tiger;
    }

    // Then trap all tigers
    board.place_goat(1);
    board.place_goat(5);
    board.place_goat(6);

    // Even though tigers are trapped, they should win because they captured 5 goats
    assert_eq!(board.get_winner(), Winner::Tigers);
    assert!(board.is_game_over());
}
