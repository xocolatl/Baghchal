use colored::Colorize;
use std::fmt::Display;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Piece {
    Tiger,
    Goat,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position(usize);

#[derive(Debug)]
struct Board {
    // 5x5 grid represented as a flat array
    cells: [Piece; 25],
    goats_in_hand: u32,  // Goats not yet placed on the board
    captured_goats: u32, // Goats captured by tigers
}

impl Board {
    fn new() -> Self {
        let mut cells = [Piece::Empty; 25];
        // Place the four tigers in their initial positions (corners)
        cells[0] = Piece::Tiger; // Top-left
        cells[4] = Piece::Tiger; // Top-right
        cells[20] = Piece::Tiger; // Bottom-left
        cells[24] = Piece::Tiger; // Bottom-right

        Board {
            cells,
            goats_in_hand: 20, // Game starts with 20 goats to place
            captured_goats: 0,
        }
    }

    fn place_goat(&mut self, position: usize) -> bool {
        if position >= self.cells.len()
            || self.cells[position] != Piece::Empty
            || self.goats_in_hand == 0
        {
            return false;
        }

        self.cells[position] = Piece::Goat;
        self.goats_in_hand -= 1;
        true
    }

    fn is_game_over(&self) -> bool {
        self.captured_goats >= 5 || (self.goats_in_hand == 0 && self.captured_goats >= 5)
    }

    fn move_tiger(&mut self, from: usize, to: usize) -> bool {
        if from >= self.cells.len() || to >= self.cells.len() {
            return false;
        }

        // Check if there's actually a tiger at the starting position
        if self.cells[from] != Piece::Tiger {
            return false;
        }

        // Check if destination is empty
        if self.cells[to] != Piece::Empty {
            return false;
        }

        // Get valid moves for this tiger
        let valid_moves = self.get_valid_tiger_moves(from);
        if !valid_moves.contains(&Position(to)) {
            return false;
        }

        // If it's a capture move (distance > 1), remove the captured goat
        if let Some(captured_pos) = self.get_captured_position(from, to) {
            self.cells[captured_pos] = Piece::Empty;
            self.captured_goats += 1;
        }

        // Make the move
        self.cells[to] = Piece::Tiger;
        self.cells[from] = Piece::Empty;
        true
    }

    fn is_diagonal_allowed(&self, pos: usize) -> bool {
        // In a 5x5 grid, diagonal moves are only allowed at these positions:
        const DIAGONAL_POSITIONS: [usize; 9] = [
            0, 2, 4, // Top row
            10, 12, 14, // Middle row
            20, 22, 24, // Bottom row
        ];
        DIAGONAL_POSITIONS.contains(&pos)
    }

    fn get_valid_tiger_moves(&self, pos: usize) -> Vec<Position> {
        let mut moves = Vec::new();
        let row = pos / 5;
        let col = pos % 5;

        // Define possible moves (adjacent positions)
        let mut adjacent_moves = vec![
            (row.wrapping_sub(1), col), // Up
            (row + 1, col),             // Down
            (row, col.wrapping_sub(1)), // Left
            (row, col + 1),             // Right
        ];

        // Only add diagonal moves if the current position allows them
        if self.is_diagonal_allowed(pos) {
            adjacent_moves.extend_from_slice(&[
                (row.wrapping_sub(1), col.wrapping_sub(1)), // Up-Left
                (row.wrapping_sub(1), col + 1),             // Up-Right
                (row + 1, col.wrapping_sub(1)),             // Down-Left
                (row + 1, col + 1),                         // Down-Right
            ]);
        }

        // Check each possible move
        for (new_row, new_col) in adjacent_moves {
            if new_row < 5 && new_col < 5 {
                let new_pos = new_row * 5 + new_col;
                // For diagonal moves, destination must also allow diagonals
                let is_diagonal = (new_row as i32 - row as i32).abs() == 1
                    && (new_col as i32 - col as i32).abs() == 1;

                if is_diagonal && !self.is_diagonal_allowed(new_pos) {
                    continue;
                }

                // Check if the move is valid (empty space)
                if self.cells[new_pos] == Piece::Empty {
                    moves.push(Position(new_pos));
                }
                // Check if we can capture a goat by jumping
                else if self.cells[new_pos] == Piece::Goat {
                    let jump_row = if new_row > row {
                        new_row + 1
                    } else {
                        new_row.wrapping_sub(1)
                    };
                    let jump_col = if new_col > col {
                        new_col + 1
                    } else {
                        new_col.wrapping_sub(1)
                    };

                    if jump_row < 5 && jump_col < 5 {
                        let jump_pos = jump_row * 5 + jump_col;
                        // For diagonal jumps, all three positions must allow diagonals
                        if is_diagonal
                            && (!self.is_diagonal_allowed(new_pos)
                                || !self.is_diagonal_allowed(jump_pos))
                        {
                            continue;
                        }
                        if self.cells[jump_pos] == Piece::Empty {
                            moves.push(Position(jump_pos));
                        }
                    }
                }
            }
        }
        moves
    }

    fn get_captured_position(&self, from: usize, to: usize) -> Option<usize> {
        let from_row = from / 5;
        let from_col = from % 5;
        let to_row = to / 5;
        let to_col = to % 5;

        // If the move is more than one step away, it's a capture move
        if (from_row as i32 - to_row as i32).abs() > 1
            || (from_col as i32 - to_col as i32).abs() > 1
        {
            // The captured position is the middle position
            let mid_row = (from_row + to_row) / 2;
            let mid_col = (from_col + to_col) / 2;
            let mid_pos = mid_row * 5 + mid_col;

            if self.cells[mid_pos] == Piece::Goat {
                return Some(mid_pos);
            }
        }
        None
    }

    fn is_valid_connection(&self, pos1: usize, pos2: usize) -> bool {
        // Always allow orthogonal connections
        let row1 = pos1 / 5;
        let col1 = pos1 % 5;
        let row2 = pos2 / 5;
        let col2 = pos2 % 5;

        // Check if positions are adjacent
        (row1 == row2 && (col1 as i32 - col2 as i32).abs() == 1)
            || (col1 == col2 && (row1 as i32 - row2 as i32).abs() == 1)
    }

    fn is_valid_diagonal(&self, pos1: usize, pos2: usize) -> bool {
        // Only allow diagonals at special positions
        self.is_diagonal_allowed(pos1) && self.is_diagonal_allowed(pos2)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, cell) in self.cells.iter().enumerate() {
            if i % 5 == 0 {
                write!(f, "   ")?; // Initial spacing
            }

            let piece = match cell {
                Piece::Tiger => "T".red().bold().to_string(),
                Piece::Goat => "G".yellow().bold().to_string(),
                Piece::Empty => "·".to_string(),
            };
            write!(f, "{piece}")?;

            if (i + 1) % 5 == 0 {
                writeln!(f)?;
            } else {
                write!(f, " ")?; // Add space between pieces for better readability
            }
        }
        Ok(())
    }
}

fn get_user_input(prompt: &str) -> Option<usize> {
    loop {
        print!("{prompt}");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input.eq_ignore_ascii_case("q") || input.eq_ignore_ascii_case("quit") {
            return None;
        }

        match input.parse() {
            Ok(num) if num < 25 => return Some(num),
            _ => println!("Please enter a valid position (0-24) or 'q' to quit"),
        }
    }
}

fn print_instructions() {
    println!("\n=== BAGHCHAL ===");
    println!("A traditional board game from Nepal");
    println!("\nBoard positions are numbered 0-24, left to right, top to bottom:");
    println!("  0  1  2  3  4");
    println!("  5  6  7  8  9");
    println!(" 10 11 12 13 14");
    println!(" 15 16 17 18 19");
    println!(" 20 21 22 23 24");
    println!("\nT = Tiger, G = Goat, · = Empty");
    println!("Enter 'q' or 'quit' at any time to exit the game");
    println!("===============\n");
}

fn main() {
    let mut board = Board::new();
    print_instructions();
    println!("Current board:");
    println!("{board}");

    // Main game loop
    let mut tigers_turn = false;
    while !board.is_game_over() {
        println!("\n{}'s turn", if tigers_turn { "Tiger" } else { "Goat" });

        if tigers_turn {
            // Tiger's turn
            let from = match get_user_input("Enter tiger position to move from (0-24): ") {
                Some(pos) => pos,
                None => break,
            };

            let to = match get_user_input("Enter position to move to (0-24): ") {
                Some(pos) => pos,
                None => break,
            };

            if !board.move_tiger(from, to) {
                println!("Invalid tiger move! Try again.");
                continue;
            }
            println!("Tiger moved! Captured goats: {}", board.captured_goats);
        } else {
            // Goat's turn
            if board.goats_in_hand > 0 {
                let pos = match get_user_input("Enter position to place goat (0-24): ") {
                    Some(pos) => pos,
                    None => break,
                };

                if !board.place_goat(pos) {
                    println!("Invalid move! Try again.");
                    continue;
                }
                println!("Goats remaining to place: {}", board.goats_in_hand);
            } else {
                println!("All goats placed! (Goat movement not implemented yet)");
                break;
            }
        }

        println!("\nCurrent board:");
        println!("{board}");
        tigers_turn = !tigers_turn;
    }

    println!("\nGame ended!");
    println!("Final board state:");
    println!("{board}");
    println!("Captured goats: {}", board.captured_goats);
}
