use baghchal::{Board, Piece, Player, Winner};
use colored::Colorize;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

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

fn parse_position(input: &str) -> Option<usize> {
    // Only accept coordinate format (A1-E5)
    if input.len() == 2 {
        let chars: Vec<char> = input.chars().collect();
        let col = chars[0].to_ascii_uppercase();
        let row = chars[1].to_digit(10);

        if let Some(row_num) = row {
            if row_num >= 1 && row_num <= 5 {
                let col_num = match col {
                    'A' => 0,
                    'B' => 1,
                    'C' => 2,
                    'D' => 3,
                    'E' => 4,
                    _ => return None,
                };
                return Some((row_num as usize - 1) * 5 + col_num);
            }
        }
    }

    None
}

fn parse_move(input: &str) -> Option<(usize, usize)> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() == 2 {
        if let (Some(from), Some(to)) = (parse_position(parts[0]), parse_position(parts[1])) {
            return Some((from, to));
        }
    }
    None
}

fn get_position(prompt: &str) -> Option<usize> {
    loop {
        if let Some(input) = get_user_input(prompt) {
            match parse_position(&input) {
                Some(pos) => return Some(pos),
                None => println!("Please enter a valid position (A1-E5)"),
            }
        } else {
            return None;
        }
    }
}

fn print_instructions() {
    println!("\n=== BAGHCHAL ===");
    println!("A traditional board game from Nepal");
    println!("\nPositions are specified using grid coordinates (A1-E5)");
    println!("T = Tiger, G = Goat, · = Empty");
    println!("Commands:");
    println!("  - To move a piece:");
    println!("    • Enter both positions at once (e.g., 'A1 A2')");
    println!("    • Or enter one position to see valid moves, then enter destination");
    println!("  - Enter a single position (e.g., 'A1') to place a goat");
    println!("  - Type 'h' or 'hint' to get a suggested move");
    println!("  - Type 'u' or 'undo' to take back the last move");
    println!("  - Type 'q' or 'quit' to exit the game");
    println!("  - Press Ctrl+C during AI's turn to interrupt");
    println!("===============\n");
}

fn configure_ai_time_limit(board: &mut Board) {
    loop {
        if let Some(input) = get_user_input("Enter AI thinking time in seconds (1-10): ") {
            if let Ok(seconds) = input.parse::<u64>() {
                if seconds >= 1 && seconds <= 10 {
                    board.set_ai_time_limit(seconds);
                    println!("AI thinking time set to {} seconds", seconds);
                    break;
                }
            }
            println!("Please enter a number between 1 and 10");
        }
    }
}

fn get_game_mode() -> (Player, Player) {
    loop {
        println!("\nSelect game mode:");
        println!("1. Human vs Human");
        println!("2. Human vs AI (Human plays Tigers)");
        println!("3. Human vs AI (Human plays Goats)");
        println!("4. AI vs AI");

        if let Some(input) = get_user_input("Enter mode (1-4): ") {
            let players = match input.as_str() {
                "1" => Some((Player::Human, Player::Human)),
                "2" => Some((Player::Human, Player::AI)),
                "3" => Some((Player::AI, Player::Human)),
                "4" => Some((Player::AI, Player::AI)),
                _ => {
                    println!("Invalid choice. Please enter 1, 2, 3, or 4.");
                    None
                }
            };

            if let Some(players) = players {
                return players;
            }
        }
    }
}

fn get_game_mode_string(tiger_player: Player, goat_player: Player) -> String {
    match (tiger_player, goat_player) {
        (Player::Human, Player::Human) => "Human vs Human".to_string(),
        (Player::Human, Player::AI) => "Human (Tigers) vs AI (Goats)".to_string(),
        (Player::AI, Player::Human) => "AI (Tigers) vs Human (Goats)".to_string(),
        (Player::AI, Player::AI) => "AI vs AI".to_string(),
    }
}

fn print_game_status(board: &Board, tigers_turn: bool, game_mode: &str) {
    println!("\n╔═══════════════════════════════════════════╗");
    println!("║ {:<41} ║", game_mode);
    println!("╟───────────────────────────────────────────╢");

    // Current turn with fixed spacing
    let turn_text = if tigers_turn {
        "Tigers".red().bold().to_string()
    } else {
        "Goats".yellow().bold().to_string()
    };
    println!("║ Current Turn: {:<38} ║", turn_text);
    println!("║ Goats in hand: {:<26} ║", board.goats_in_hand);
    println!("║ Captured goats: {:<25} ║", board.captured_goats);
    println!("╚═══════════════════════════════════════════╝\n");
}

fn get_coordinate_string(pos: usize) -> String {
    let row = pos / 5 + 1;
    let col = (pos % 5) as u8 + b'A';
    format!("{}{}", col as char, row)
}

fn print_game_end_screen(board: &Board, winner: Winner, interrupted: bool, game_mode: &str) {
    println!("\n╔═════════════════════════════════════════════════╗");
    println!("║               🎮 GAME OVER! 🎮                  ║");
    println!("╟─────────────────────────────────────────────────╢");
    println!("║ Mode: {:<41} ║", game_mode);
    println!("╟─────────────────────────────────────────────────╢");

    if interrupted {
        println!("║           🛑 Game was interrupted! 🛑            ║");
    } else {
        match winner {
            Winner::Tigers => {
                println!("║          🐯 The Tigers are victorious! 🐯         ║");
                println!("╟─────────────────────────────────────────────────╢");
                println!("║ Goats captured: {:<33} ║", board.captured_goats);
            }
            Winner::Goats => {
                println!("║           🐐 The Goats have won! 🐐             ║");
                println!("╟─────────────────────────────────────────────────╢");
                println!("║ Tigers trapped: All                             ║");
            }
            Winner::None => {
                println!("║              ⭐ Game ended! ⭐                   ║");
            }
        }
    }

    println!("╟─────────────────────────────────────────────────╢");
    println!("║ Final board state:                              ║");
    println!("╚═════════════════════════════════════════════════╝\n");

    println!("{}", board.display_with_hints());

    println!("\nThanks for playing! Type 'q' to quit or press Enter to play again.");
}

fn main() {
    loop {
        let mut board = Board::new();
        print_instructions();

        let (tiger_player, goat_player) = get_game_mode();
        let playing_against_ai = tiger_player != goat_player;
        let game_mode = get_game_mode_string(tiger_player, goat_player);

        // Configure AI time limit if playing against AI
        if playing_against_ai || (tiger_player == Player::AI && goat_player == Player::AI) {
            configure_ai_time_limit(&mut board);
        }

        // Set up Ctrl+C handler
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();
        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");

        println!("\nStarting game...");
        println!("Current board:");
        println!("{}", board.display_with_hints());

        // Main game loop
        let mut tigers_turn = false;
        while !board.is_game_over() && running.load(Ordering::SeqCst) {
            print_game_status(&board, tigers_turn, &game_mode);
            println!("{}", board.display_with_hints());

            let current_player = if tigers_turn {
                tiger_player
            } else {
                goat_player
            };

            match current_player {
                Player::Human => {
                    if let Some(input) =
                        get_user_input("Enter command (position(s) A1-E5, hint, undo, or quit): ")
                    {
                        if input.eq_ignore_ascii_case("h") || input.eq_ignore_ascii_case("hint") {
                            println!("\n🤔 Thinking of a good move...");

                            // Create a temporary board for AI analysis
                            let mut temp_board = board.clone();
                            let success = if tigers_turn {
                                temp_board.ai_move_tiger()
                            } else {
                                temp_board.ai_move_goat()
                            };

                            if success {
                                // Compare the boards to find what move was made
                                for i in 0..25 {
                                    if board.cells[i] != temp_board.cells[i] {
                                        if temp_board.cells[i] == Piece::Empty {
                                            // This was the 'from' position
                                            print!(
                                                "\n💡 Suggested move: {}",
                                                get_coordinate_string(i)
                                            );
                                        } else if board.cells[i] == Piece::Empty {
                                            // This was the 'to' position
                                            println!(" {}", get_coordinate_string(i));
                                        }
                                    }
                                }
                            } else {
                                println!("\n😕 No good moves available!");
                            }
                            continue;
                        }
                        if input.eq_ignore_ascii_case("u") || input.eq_ignore_ascii_case("undo") {
                            if board.can_undo() {
                                // If playing against AI, undo both moves
                                if playing_against_ai {
                                    if board.can_undo() {
                                        board.undo(); // Undo AI's move
                                        if board.can_undo() {
                                            board.undo(); // Undo player's move
                                            println!(
                                                "\nUndid both your move and the AI's response!"
                                            );
                                        } else {
                                            println!("\nUndid the AI's move!");
                                        }
                                    }
                                } else {
                                    board.undo(); // Just undo one move in human vs human
                                    println!("\nMove undone!");
                                }
                                tigers_turn = !tigers_turn;
                                println!("Current board:");
                                println!("{}", board.display_with_hints());
                                continue;
                            } else {
                                println!("No moves to undo!");
                                continue;
                            }
                        }

                        if tigers_turn {
                            // Tiger's turn
                            if let Some((from, to)) = parse_move(&input) {
                                // Two-step move provided
                                if board.cells[from] != Piece::Tiger {
                                    println!(
                                        "No tiger at position {}! Try again.",
                                        get_coordinate_string(from)
                                    );
                                    continue;
                                }

                                if !board.move_tiger(from, to) {
                                    println!("Invalid tiger move! Try again.");
                                    continue;
                                }
                                println!("Tiger moved! Captured goats: {}", board.captured_goats);
                            } else if let Some(from) = parse_position(&input) {
                                // Single-step move: first select the piece
                                if board.cells[from] != Piece::Tiger {
                                    println!(
                                        "No tiger at position {}! Try again.",
                                        get_coordinate_string(from)
                                    );
                                    continue;
                                }

                                // Show valid moves for selected tiger
                                board.select_position(from);
                                println!("\nValid moves marked with •");
                                println!("{}", board.display_with_hints());

                                let to = match get_position("Enter position to move to (A1-E5): ") {
                                    Some(pos) => pos,
                                    None => break,
                                };

                                if !board.move_tiger(from, to) {
                                    println!("Invalid tiger move! Try again.");
                                    board.clear_selection();
                                    continue;
                                }
                                println!("Tiger moved! Captured goats: {}", board.captured_goats);
                                board.clear_selection();
                            } else {
                                println!("Invalid command! Please enter position(s) (e.g., 'A1' or 'A1 A2'), 'h' for hint, 'u' for undo, or 'q' to quit");
                                continue;
                            }
                        } else {
                            // Goat's turn
                            if board.goats_in_hand > 0 {
                                if let Some(pos) = parse_position(&input) {
                                    if !board.place_goat(pos) {
                                        println!("Invalid move! Try again.");
                                        continue;
                                    }
                                    println!("Goats remaining to place: {}", board.goats_in_hand);
                                } else {
                                    println!("Invalid command! Please enter a position (A1-E5), 'h' for hint, 'u' for undo, or 'q' to quit");
                                    continue;
                                }
                            } else {
                                if let Some((from, to)) = parse_move(&input) {
                                    // Two-step move provided
                                    if board.cells[from] != Piece::Goat {
                                        println!(
                                            "No goat at position {}! Try again.",
                                            get_coordinate_string(from)
                                        );
                                        continue;
                                    }

                                    if !board.move_goat(from, to) {
                                        println!("Invalid goat move! Try again.");
                                        continue;
                                    }
                                    println!("Goat moved!");
                                } else if let Some(from) = parse_position(&input) {
                                    // Single-step move: first select the piece
                                    if board.cells[from] != Piece::Goat {
                                        println!(
                                            "No goat at position {}! Try again.",
                                            get_coordinate_string(from)
                                        );
                                        continue;
                                    }

                                    // Show valid moves for selected goat
                                    board.select_position(from);
                                    println!("\nValid moves marked with •");
                                    println!("{}", board.display_with_hints());

                                    let to =
                                        match get_position("Enter position to move to (A1-E5): ") {
                                            Some(pos) => pos,
                                            None => break,
                                        };

                                    if !board.move_goat(from, to) {
                                        println!("Invalid goat move! Try again.");
                                        board.clear_selection();
                                        continue;
                                    }
                                    println!("Goat moved!");
                                    board.clear_selection();
                                } else {
                                    println!("Invalid command! Please enter position(s) (e.g., 'A1' or 'A1 A2'), 'h' for hint, 'u' for undo, or 'q' to quit");
                                    continue;
                                }
                            }
                        }
                    } else {
                        break;
                    }
                }
                Player::AI => {
                    println!("AI is thinking... (Press Ctrl+C to interrupt)");

                    // Reset the running flag in case it was interrupted before
                    running.store(true, Ordering::SeqCst);

                    let start_time = std::time::Instant::now();
                    let success = if tigers_turn {
                        board.ai_move_tiger()
                    } else {
                        board.ai_move_goat()
                    };

                    // If we were interrupted, undo the move and break
                    if !running.load(Ordering::SeqCst) {
                        println!("\nAI move interrupted!");
                        if board.can_undo() {
                            board.undo();
                        }
                        break;
                    }

                    // Add a small delay if the move was very quick
                    let elapsed = start_time.elapsed();
                    if elapsed < Duration::from_millis(500) {
                        std::thread::sleep(Duration::from_millis(500) - elapsed);
                    }

                    if !success {
                        println!("AI couldn't make a move!");
                        break;
                    }

                    if tigers_turn {
                        println!("Tiger moved! Captured goats: {}", board.captured_goats);
                    } else if board.goats_in_hand > 0 {
                        println!("Goat placed! Remaining to place: {}", board.goats_in_hand);
                    } else {
                        println!("Goat moved!");
                    }
                }
            }

            println!("\nCurrent board:");
            println!("{}", board.display_with_hints());
            tigers_turn = !tigers_turn;
        }

        let interrupted = !running.load(Ordering::SeqCst);
        let winner = board.get_winner();

        print_game_end_screen(&board, winner, interrupted, &game_mode);

        // Ask to play again
        if let Some(input) = get_user_input("") {
            if input.eq_ignore_ascii_case("q") || input.eq_ignore_ascii_case("quit") {
                break;
            }
        } else {
            break;
        }
    }
}
