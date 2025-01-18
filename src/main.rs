use baghchal::{Board, Piece, Winner};
use std::io::{self, Write};

fn get_user_input(prompt: &str) -> Option<String> {
    loop {
        print!("{prompt}");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input.is_empty() {
            println!("Please enter a command");
            continue;
        }
        if input.eq_ignore_ascii_case("q") || input.eq_ignore_ascii_case("quit") {
            return None;
        }
        return Some(input.to_string());
    }
}

fn get_position(prompt: &str) -> Option<usize> {
    loop {
        if let Some(input) = get_user_input(prompt) {
            match input.parse() {
                Ok(num) if num < 25 => return Some(num),
                _ => println!("Please enter a valid position (0-24)"),
            }
        } else {
            return None;
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
    println!("Commands:");
    println!("  - Enter a number (0-24) to select a position");
    println!("  - Type 'u' or 'undo' to take back the last move");
    println!("  - Type 'q' or 'quit' to exit the game");
    println!("===============\n");
}

fn main() {
    let mut board = Board::new();
    print_instructions();
    println!("Current board:");
    println!("{}", board.display_with_hints());

    // Main game loop
    let mut tigers_turn = false;
    while !board.is_game_over() {
        println!("\n{}'s turn", if tigers_turn { "Tiger" } else { "Goat" });

        if let Some(input) = get_user_input("Enter command (position, undo, or quit): ") {
            if input.eq_ignore_ascii_case("u") || input.eq_ignore_ascii_case("undo") {
                if board.can_undo() {
                    board.undo();
                    tigers_turn = !tigers_turn;
                    println!("\nMove undone!");
                    println!("Current board:");
                    println!("{}", board.display_with_hints());
                    continue;
                } else {
                    println!("No moves to undo!");
                    continue;
                }
            }

            let position = match input.parse::<usize>() {
                Ok(pos) if pos < 25 => pos,
                _ => {
                    println!("Invalid command! Please enter a position (0-24), 'u' for undo, or 'q' to quit");
                    continue;
                }
            };

            if tigers_turn {
                // Tiger's turn
                if board.cells[position] != Piece::Tiger {
                    println!("No tiger at that position! Try again.");
                    continue;
                }

                // Show valid moves for selected tiger
                board.select_position(position);
                println!("\nValid moves marked with •");
                println!("{}", board.display_with_hints());

                let to = match get_position("Enter position to move to (0-24): ") {
                    Some(pos) => pos,
                    None => break,
                };

                if !board.move_tiger(position, to) {
                    println!("Invalid tiger move! Try again.");
                    board.clear_selection();
                    continue;
                }
                println!("Tiger moved! Captured goats: {}", board.captured_goats);
                board.clear_selection();
            } else {
                // Goat's turn
                if board.goats_in_hand > 0 {
                    if !board.place_goat(position) {
                        println!("Invalid move! Try again.");
                        continue;
                    }
                    println!("Goats remaining to place: {}", board.goats_in_hand);
                } else {
                    if board.cells[position] != Piece::Goat {
                        println!("No goat at that position! Try again.");
                        continue;
                    }

                    // Show valid moves for selected goat
                    board.select_position(position);
                    println!("\nValid moves marked with •");
                    println!("{}", board.display_with_hints());

                    let to = match get_position("Enter position to move to (0-24): ") {
                        Some(pos) => pos,
                        None => break,
                    };

                    if !board.move_goat(position, to) {
                        println!("Invalid goat move! Try again.");
                        board.clear_selection();
                        continue;
                    }
                    println!("Goat moved!");
                    board.clear_selection();
                }
            }

            println!("\nCurrent board:");
            println!("{}", board.display_with_hints());
            tigers_turn = !tigers_turn;
        } else {
            break;
        }
    }

    println!("\nGame ended!");
    println!("Final board state:");
    println!("{}", board.display_with_hints());
    println!("Captured goats: {}", board.captured_goats);

    match board.get_winner() {
        Winner::Tigers => println!("Tigers win by capturing {} goats!", board.captured_goats),
        Winner::Goats => println!("Goats win by trapping all tigers!"),
        Winner::None => println!("Game ended without a winner."),
    }
}
