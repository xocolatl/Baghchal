use baghchal::Board;
use std::io::{self, Write};

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
