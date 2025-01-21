use colored::Colorize;
use std::fmt::Display;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Tiger,
    Goat,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position(pub usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Winner {
    Tigers,
    Goats,
    None,
}

#[derive(Debug, Clone, Copy)]
pub enum Move {
    PlaceGoat {
        position: usize,
    },
    MoveGoat {
        from: usize,
        to: usize,
    },
    MoveTiger {
        from: usize,
        to: usize,
        captured_position: Option<usize>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    Human,
    AI,
}

#[derive(Debug, Clone)]
pub struct Board {
    pub cells: [Piece; 25],
    pub goats_in_hand: u32,
    pub captured_goats: u32,
    pub selected_position: Option<usize>,
    move_history: Vec<Move>, // Track all moves
}

impl Board {
    pub fn new() -> Self {
        let mut cells = [Piece::Empty; 25];
        cells[0] = Piece::Tiger;
        cells[4] = Piece::Tiger;
        cells[20] = Piece::Tiger;
        cells[24] = Piece::Tiger;

        Board {
            cells,
            goats_in_hand: 20,
            captured_goats: 0,
            selected_position: None,
            move_history: Vec::new(),
        }
    }

    pub fn display_with_hints(&self) -> String {
        let mut output = String::new();

        // Add column labels (A-E)
        output.push_str("     A   B   C   D   E\n");

        // Top border
        output.push_str("   ┌───┬───┬───┬───┬───┐\n");

        for row in 0..5 {
            // Row number
            output.push_str(&format!(" {} │", row + 1));

            for col in 0..5 {
                let pos = row * 5 + col;
                let piece = match self.cells[pos] {
                    Piece::Empty => {
                        if self.selected_position.is_some()
                            && self.is_valid_move(self.selected_position.unwrap(), pos)
                        {
                            "•".bright_green()
                        } else if self.is_diagonal_allowed(pos) {
                            "×".bright_black()
                        } else {
                            " ".normal()
                        }
                    }
                    Piece::Goat => "G".bright_yellow(),
                    Piece::Tiger => "T".bright_red(),
                };

                output.push_str(&format!(" {} │", piece));
            }
            output.push('\n');

            // Add horizontal lines between rows, except for the last row
            if row < 4 {
                output.push_str("   ├───┼───┼───┼───┼───┤\n");
            }
        }

        // Bottom border
        output.push_str("   └───┴───┴───┴───┴───┘\n");

        output
    }

    pub fn select_position(&mut self, pos: usize) -> bool {
        if pos >= self.cells.len() {
            return false;
        }
        self.selected_position = Some(pos);
        true
    }

    pub fn clear_selection(&mut self) {
        self.selected_position = None;
    }

    pub fn place_goat(&mut self, position: usize) -> bool {
        if position >= self.cells.len()
            || self.cells[position] != Piece::Empty
            || self.goats_in_hand == 0
        {
            return false;
        }

        self.cells[position] = Piece::Goat;
        self.goats_in_hand -= 1;
        self.move_history.push(Move::PlaceGoat { position });
        true
    }

    pub fn is_game_over(&self) -> bool {
        self.get_winner() != Winner::None
    }

    pub fn get_winner(&self) -> Winner {
        // Tigers win if they've captured 5 or more goats
        if self.captured_goats >= 5 {
            return Winner::Tigers;
        }

        // Check if all tigers are trapped
        let tiger_positions: Vec<usize> = self
            .cells
            .iter()
            .enumerate()
            .filter(|(_, &piece)| piece == Piece::Tiger)
            .map(|(pos, _)| pos)
            .collect();

        // If any tiger can move, game is not over
        for &pos in &tiger_positions {
            if !self.get_valid_tiger_moves(pos).is_empty() {
                return Winner::None;
            }
        }

        // If we get here, no tiger can move
        Winner::Goats
    }

    pub fn move_tiger(&mut self, from: usize, to: usize) -> bool {
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
        let captured_position = self.get_captured_position(from, to);
        if let Some(captured_pos) = captured_position {
            self.cells[captured_pos] = Piece::Empty;
            self.captured_goats += 1;
        }

        // Make the move
        self.cells[to] = Piece::Tiger;
        self.cells[from] = Piece::Empty;
        self.move_history.push(Move::MoveTiger {
            from,
            to,
            captured_position,
        });
        true
    }

    pub fn is_diagonal_allowed(&self, pos: usize) -> bool {
        matches!(
            pos,
            0 | 2 | 4 | 6 | 8 | 10 | 12 | 14 | 16 | 18 | 20 | 22 | 24
        )
    }

    pub fn get_valid_tiger_moves(&self, pos: usize) -> Vec<Position> {
        let mut moves = Vec::new();
        let row = pos / 5;
        let col = pos % 5;

        // Define possible moves (adjacent positions and potential jumps)
        let mut possible_moves = vec![
            // Adjacent moves
            (row.wrapping_sub(1), col), // Up
            (row + 1, col),             // Down
            (row, col.wrapping_sub(1)), // Left
            (row, col + 1),             // Right
            // Jump moves
            (row.wrapping_sub(2), col), // Jump Up
            (row + 2, col),             // Jump Down
            (row, col.wrapping_sub(2)), // Jump Left
            (row, col + 2),             // Jump Right
        ];

        // Only add diagonal moves if the current position allows them
        if self.is_diagonal_allowed(pos) {
            possible_moves.extend_from_slice(&[
                // Adjacent diagonal moves
                (row.wrapping_sub(1), col.wrapping_sub(1)), // Up-Left
                (row.wrapping_sub(1), col + 1),             // Up-Right
                (row + 1, col.wrapping_sub(1)),             // Down-Left
                (row + 1, col + 1),                         // Down-Right
                // Jump diagonal moves
                (row.wrapping_sub(2), col.wrapping_sub(2)), // Jump Up-Left
                (row.wrapping_sub(2), col + 2),             // Jump Up-Right
                (row + 2, col.wrapping_sub(2)),             // Jump Down-Left
                (row + 2, col + 2),                         // Jump Down-Right
            ]);
        }

        // Check each possible move
        for (new_row, new_col) in possible_moves {
            if new_row < 5 && new_col < 5 {
                let new_pos = new_row * 5 + new_col;

                // Calculate if this is a jump move
                let row_diff = (new_row as i32 - row as i32).abs();
                let col_diff = (new_col as i32 - col as i32).abs();
                let is_jump = row_diff == 2 || col_diff == 2;
                let is_diagonal = row_diff == col_diff;

                // Skip invalid diagonal moves
                if is_diagonal && !self.is_diagonal_allowed(new_pos) {
                    continue;
                }

                // For jump moves, check if there's a goat to capture
                if is_jump {
                    let mid_row = (row + new_row) / 2;
                    let mid_col = (col + new_col) / 2;
                    let mid_pos = mid_row * 5 + mid_col;

                    // For diagonal jumps, all positions must allow diagonals
                    if is_diagonal && !self.is_diagonal_allowed(mid_pos) {
                        continue;
                    }

                    // Can only jump if there's a goat in the middle and the destination is empty
                    if self.cells[mid_pos] == Piece::Goat && self.cells[new_pos] == Piece::Empty {
                        moves.push(Position(new_pos));
                    }
                } else if self.cells[new_pos] == Piece::Empty {
                    // For non-jump moves, just check if the destination is empty
                    moves.push(Position(new_pos));
                }
            }
        }
        moves
    }

    pub fn get_captured_position(&self, from: usize, to: usize) -> Option<usize> {
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

    pub fn move_goat(&mut self, from: usize, to: usize) -> bool {
        if from >= self.cells.len() || to >= self.cells.len() {
            return false;
        }

        // Check if there's actually a goat at the starting position
        if self.cells[from] != Piece::Goat {
            return false;
        }

        // Check if destination is empty
        if self.cells[to] != Piece::Empty {
            return false;
        }

        // Get valid moves for this goat
        let valid_moves = self.get_valid_goat_moves(from);
        if !valid_moves.contains(&Position(to)) {
            return false;
        }

        // Make the move
        self.cells[to] = Piece::Goat;
        self.cells[from] = Piece::Empty;
        self.move_history.push(Move::MoveGoat { from, to });
        true
    }

    pub fn get_valid_goat_moves(&self, pos: usize) -> Vec<Position> {
        let mut moves = Vec::new();
        let row = pos / 5;
        let col = pos % 5;

        // Define possible moves (adjacent positions only)
        let mut possible_moves = vec![
            (row.wrapping_sub(1), col), // Up
            (row + 1, col),             // Down
            (row, col.wrapping_sub(1)), // Left
            (row, col + 1),             // Right
        ];

        // Only add diagonal moves if the current position allows them
        if self.is_diagonal_allowed(pos) {
            possible_moves.extend_from_slice(&[
                (row.wrapping_sub(1), col.wrapping_sub(1)), // Up-Left
                (row.wrapping_sub(1), col + 1),             // Up-Right
                (row + 1, col.wrapping_sub(1)),             // Down-Left
                (row + 1, col + 1),                         // Down-Right
            ]);
        }

        // Check each possible move
        for (new_row, new_col) in possible_moves {
            if new_row < 5 && new_col < 5 {
                let new_pos = new_row * 5 + new_col;

                // Check if this is a diagonal move
                let row_diff = (new_row as i32 - row as i32).abs();
                let col_diff = (new_col as i32 - col as i32).abs();
                let is_diagonal = row_diff == col_diff;

                // Skip invalid diagonal moves
                if is_diagonal && !self.is_diagonal_allowed(new_pos) {
                    continue;
                }

                // Check if the destination is empty
                if self.cells[new_pos] == Piece::Empty {
                    moves.push(Position(new_pos));
                }
            }
        }
        moves
    }

    pub fn can_undo(&self) -> bool {
        !self.move_history.is_empty()
    }

    pub fn undo(&mut self) -> bool {
        if let Some(last_move) = self.move_history.pop() {
            match last_move {
                Move::PlaceGoat { position } => {
                    self.cells[position] = Piece::Empty;
                    self.goats_in_hand += 1;
                }
                Move::MoveGoat { from, to } => {
                    self.cells[from] = Piece::Goat;
                    self.cells[to] = Piece::Empty;
                }
                Move::MoveTiger {
                    from,
                    to,
                    captured_position,
                } => {
                    self.cells[from] = Piece::Tiger;
                    self.cells[to] = Piece::Empty;
                    if let Some(captured_pos) = captured_position {
                        self.cells[captured_pos] = Piece::Goat;
                        self.captured_goats -= 1;
                    }
                }
            }
            self.selected_position = None;
            true
        } else {
            false
        }
    }

    pub fn get_all_valid_tiger_moves(&self) -> Vec<(usize, usize)> {
        let mut all_moves = Vec::new();

        // Find all tigers
        for (pos, &piece) in self.cells.iter().enumerate() {
            if piece == Piece::Tiger {
                // Get valid moves for this tiger
                for move_pos in self.get_valid_tiger_moves(pos) {
                    all_moves.push((pos, move_pos.0));
                }
            }
        }

        all_moves
    }

    pub fn get_all_valid_goat_moves(&self) -> Vec<(usize, usize)> {
        let mut all_moves = Vec::new();

        if self.goats_in_hand > 0 {
            // Can place a new goat
            for pos in 0..25 {
                if self.cells[pos] == Piece::Empty {
                    all_moves.push((pos, pos)); // From and to are same for placement
                }
            }
            return all_moves; // Return early to avoid mixing placement and movement
        }

        // Move existing goats
        for (pos, &piece) in self.cells.iter().enumerate() {
            if piece == Piece::Goat {
                // Get valid moves for this goat
                for move_pos in self.get_valid_goat_moves(pos) {
                    all_moves.push((pos, move_pos.0));
                }
            }
        }

        all_moves
    }

    fn evaluate_position(&self) -> i32 {
        // If game is over, return a large value
        match self.get_winner() {
            Winner::Tigers => return 10000,
            Winner::Goats => return -10000,
            Winner::None => {}
        }

        let mut score = 0;

        // Each captured goat is worth 100 points
        score += self.captured_goats as i32 * 100;

        // Each trapped tiger is worth -50 points
        let trapped_tigers = self
            .cells
            .iter()
            .enumerate()
            .filter(|(_, &piece)| piece == Piece::Tiger)
            .filter(|&(pos, _)| self.get_valid_tiger_moves(pos).is_empty())
            .count();
        score -= trapped_tigers as i32 * 50;

        // Each goat in a strategic position is worth -10 points
        let strategic_positions = [
            12, // Center
            6, 8, 16, 18, // Diagonal positions
            7, 11, 13, 17, // Adjacent to center
        ];
        let strategic_goats = strategic_positions
            .iter()
            .filter(|&&pos| self.cells[pos] == Piece::Goat)
            .count();
        score -= strategic_goats as i32 * 10;

        // Each goat that can be captured is worth 20 points
        let capturable_goats = self
            .cells
            .iter()
            .enumerate()
            .filter(|(_, &piece)| piece == Piece::Tiger)
            .flat_map(|(pos, _)| self.get_valid_tiger_moves(pos))
            .filter(|move_pos| {
                let from = self
                    .cells
                    .iter()
                    .position(|&piece| piece == Piece::Tiger)
                    .unwrap_or(0);
                self.get_captured_position(from, move_pos.0).is_some()
            })
            .count();
        score += capturable_goats as i32 * 20;

        score
    }

    pub fn ai_move_tiger(&mut self) -> bool {
        let moves = self.get_all_valid_tiger_moves();
        if moves.is_empty() {
            return false;
        }

        let mut best_move = None;
        let mut best_score = i32::MIN;
        let time_limit = Duration::from_secs(2); // 2 seconds time limit
        let start_time = Instant::now();
        let mut current_depth = 1;

        // Iterative deepening
        while start_time.elapsed() < time_limit {
            let mut depth_best_move = None;
            let mut depth_best_score = i32::MIN;
            let mut search_complete = true;

            for (from, to) in moves.iter() {
                // Check if we've run out of time
                if start_time.elapsed() >= time_limit {
                    search_complete = false;
                    break;
                }

                // Make move
                let captured_pos = self.get_captured_position(*from, *to);
                let original_from = self.cells[*from];
                let original_to = self.cells[*to];
                let mut original_captured = None;
                if let Some(pos) = captured_pos {
                    original_captured = Some((pos, self.cells[pos]));
                    self.cells[pos] = Piece::Empty;
                    self.captured_goats += 1;
                }
                self.cells[*from] = Piece::Empty;
                self.cells[*to] = Piece::Tiger;

                // Evaluate position
                let score = self.minimax(
                    current_depth - 1,
                    i32::MIN,
                    i32::MAX,
                    false,
                    start_time,
                    time_limit,
                );

                // Undo move
                self.cells[*from] = original_from;
                self.cells[*to] = original_to;
                if let Some((pos, piece)) = original_captured {
                    self.cells[pos] = piece;
                    self.captured_goats -= 1;
                }

                // Update best move for current depth
                if score > depth_best_score {
                    depth_best_score = score;
                    depth_best_move = Some((*from, *to));
                }
            }

            // Only update the overall best move if we completed the search at this depth
            if search_complete {
                best_move = depth_best_move;
                best_score = depth_best_score;
                current_depth += 1;
            } else {
                break;
            }
        }

        // Make the best move found
        if let Some((from, to)) = best_move {
            return self.move_tiger(from, to);
        }

        false
    }

    pub fn ai_move_goat(&mut self) -> bool {
        let time_limit = Duration::from_secs(2); // 2 seconds time limit
        let start_time = Instant::now();
        let mut current_depth = 1;
        let mut best_move = None;
        let mut best_score = i32::MAX;

        while start_time.elapsed() < time_limit {
            let mut depth_best_move = None;
            let mut depth_best_score = i32::MAX;
            let mut search_complete = true;

            if self.goats_in_hand > 0 {
                // Try each empty position for placement
                for pos in 0..25 {
                    if start_time.elapsed() >= time_limit {
                        search_complete = false;
                        break;
                    }

                    if self.cells[pos] == Piece::Empty {
                        // Make move
                        self.cells[pos] = Piece::Goat;
                        self.goats_in_hand -= 1;

                        // Evaluate position
                        let score = self.minimax(
                            current_depth - 1,
                            i32::MIN,
                            i32::MAX,
                            true,
                            start_time,
                            time_limit,
                        );

                        // Undo move
                        self.cells[pos] = Piece::Empty;
                        self.goats_in_hand += 1;

                        // Update best move for current depth
                        if score < depth_best_score {
                            depth_best_score = score;
                            depth_best_move = Some((pos, pos));
                        }
                    }
                }
            } else {
                // Move existing goats
                let moves = self.get_all_valid_goat_moves();
                for (from, to) in moves {
                    if start_time.elapsed() >= time_limit {
                        search_complete = false;
                        break;
                    }

                    // Make move
                    let original_from = self.cells[from];
                    let original_to = self.cells[to];
                    self.cells[from] = Piece::Empty;
                    self.cells[to] = Piece::Goat;

                    // Evaluate position
                    let score = self.minimax(
                        current_depth - 1,
                        i32::MIN,
                        i32::MAX,
                        true,
                        start_time,
                        time_limit,
                    );

                    // Undo move
                    self.cells[from] = original_from;
                    self.cells[to] = original_to;

                    // Update best move for current depth
                    if score < depth_best_score {
                        depth_best_score = score;
                        depth_best_move = Some((from, to));
                    }
                }
            }

            // Only update the overall best move if we completed the search at this depth
            if search_complete {
                best_move = depth_best_move;
                best_score = depth_best_score;
                current_depth += 1;
            } else {
                break;
            }
        }

        // Make the best move found
        if let Some((from, to)) = best_move {
            if from == to {
                return self.place_goat(from);
            } else {
                return self.move_goat(from, to);
            }
        }

        false
    }

    fn minimax(
        &mut self,
        depth: i32,
        mut alpha: i32,
        mut beta: i32,
        is_maximizing: bool,
        start_time: Instant,
        time_limit: Duration,
    ) -> i32 {
        // Check if we've run out of time
        if start_time.elapsed() >= time_limit {
            return self.evaluate_position();
        }

        if depth == 0 || self.is_game_over() {
            return self.evaluate_position();
        }

        if is_maximizing {
            // Tiger's turn (maximizing)
            let mut max_eval = i32::MIN;
            let moves = self.get_all_valid_tiger_moves();

            for (from, to) in moves {
                // Make move
                let captured_pos = self.get_captured_position(from, to);
                let original_from = self.cells[from];
                let original_to = self.cells[to];
                let mut original_captured = None;
                if let Some(pos) = captured_pos {
                    original_captured = Some((pos, self.cells[pos]));
                    self.cells[pos] = Piece::Empty;
                    self.captured_goats += 1;
                }
                self.cells[from] = Piece::Empty;
                self.cells[to] = Piece::Tiger;

                // Recursive evaluation
                let eval = self.minimax(depth - 1, alpha, beta, false, start_time, time_limit);

                // Undo move
                self.cells[from] = original_from;
                self.cells[to] = original_to;
                if let Some((pos, piece)) = original_captured {
                    self.cells[pos] = piece;
                    self.captured_goats -= 1;
                }

                max_eval = max_eval.max(eval);
                alpha = alpha.max(eval);
                if beta <= alpha {
                    break; // Beta cutoff
                }
            }
            max_eval
        } else {
            // Goat's turn (minimizing)
            let mut min_eval = i32::MAX;
            let moves = self.get_all_valid_goat_moves();

            for (from, to) in moves {
                // Make move
                let original_from = self.cells[from];
                let original_to = self.cells[to];
                if from == to {
                    // Placing a new goat
                    self.cells[to] = Piece::Goat;
                    self.goats_in_hand -= 1;
                } else {
                    // Moving an existing goat
                    self.cells[from] = Piece::Empty;
                    self.cells[to] = Piece::Goat;
                }

                // Recursive evaluation
                let eval = self.minimax(depth - 1, alpha, beta, true, start_time, time_limit);

                // Undo move
                if from == to {
                    self.cells[to] = Piece::Empty;
                    self.goats_in_hand += 1;
                } else {
                    self.cells[from] = original_from;
                    self.cells[to] = original_to;
                }

                min_eval = min_eval.min(eval);
                beta = beta.min(eval);
                if beta <= alpha {
                    break; // Alpha cutoff
                }
            }
            min_eval
        }
    }

    fn is_valid_move(&self, _from: usize, to: usize) -> bool {
        if let Some(selected) = self.selected_position {
            match self.cells[selected] {
                Piece::Tiger => self.get_valid_tiger_moves(selected).contains(&Position(to)),
                Piece::Goat => {
                    if self.goats_in_hand > 0 {
                        self.get_all_valid_goat_placements().contains(&Position(to))
                    } else {
                        self.get_valid_goat_moves(selected).contains(&Position(to))
                    }
                }
                Piece::Empty => false,
            }
        } else {
            false
        }
    }

    fn get_all_valid_goat_placements(&self) -> Vec<Position> {
        (0..25)
            .filter(|&pos| self.cells[pos] == Piece::Empty)
            .map(Position)
            .collect()
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
