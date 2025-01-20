# Baghchal

A Rust implementation of Baghchal (बाघचाल), a traditional board game from Nepal. The game is played between two players, where one controls four tigers trying to capture goats, and the other controls twenty goats trying to trap the tigers.

This entire project was generated through natural language prompts to Claude 3.5 Sonnet (Anthropic) via the Cursor IDE, without any direct human coding. All game logic, AI implementation, and user interface were created through conversational prompts and AI-assisted development. The development process included implementing game rules, board representation, move validation, AI opponent using minimax with alpha-beta pruning, and a colored terminal interface. Each feature was iteratively developed and refined through natural language conversation, with Claude generating the code, fixing bugs, and making improvements based on feedback. The source code and commit history demonstrate this unique development process where a complex game implementation emerged purely from human-AI collaboration, showcasing the capabilities of modern AI coding assistants.

## Game Rules

### Board
- The game is played on a 5×5 grid with pieces placed at intersections
- The board has both orthogonal (horizontal/vertical) and diagonal lines
- Diagonal moves are only allowed at positions where diagonal lines are drawn

### Pieces
- Tigers (T): 4 pieces, start in the corners
- Goats (G): 20 pieces, start off the board
- Empty positions are marked with dots (·)

### Gameplay
1. Players take turns, with Goats moving first
2. Goat's turn:
   - Initially: Place a new goat on any empty position
   - After all goats are placed: Move one goat to an adjacent empty position
3. Tiger's turn:
   - Move to an adjacent empty position
   - OR capture a goat by jumping over it to an empty position beyond
4. Movement rules:
   - Both pieces can move along lines to adjacent intersections
   - Diagonal moves are only allowed where diagonal lines exist
   - Tigers can capture goats by jumping over them to an empty space

### Winning Conditions
- Tigers win by capturing 5 goats
- Goats win by trapping all tigers (no legal moves available)

## Features

- Full implementation of Baghchal rules
- Multiple game modes:
  - Human vs Human
  - Human vs AI (play as either Tigers or Goats)
  - AI vs AI
- Smart AI using minimax algorithm with alpha-beta pruning
- Colored terminal interface
- Move validation and hints
- Undo functionality
  - Single move undo in Human vs Human
  - Two-move undo in Human vs AI (undoes both your move and AI's response)
- Help command to show position numbers
- Ability to interrupt AI's move with Ctrl+C

## Installation

1. Make sure you have Rust installed (https://rustup.rs/)
2. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/baghchal.git
   cd baghchal
   ```
3. Build and run:
   ```bash
   cargo run
   ```

## How to Play

1. Start the game by running `cargo run`
2. Select a game mode (1-4)
3. On your turn:
   - Enter a position number (0-24) to select a piece
   - For tigers/moved goats: Enter another position to move to
   - Valid moves will be shown with • markers
4. Special commands:
   - Type 'h' or 'help' to show position numbers
   - Type 'u' or 'undo' to take back moves
   - Type 'q' or 'quit' to exit
   - Press Ctrl+C during AI's turn to interrupt

## Development

The game is written in Rust and uses the following crates:
- `colored`: For terminal colors
- `rand`: For random number generation
- `ctrlc`: For handling interrupt signals

## Contributing

Contributions are welcome! Feel free to submit issues and pull requests.

## License

This project is open source and available under the MIT License.
