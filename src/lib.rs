use colored::Colorize;
use std::fmt::Display;

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

#[derive(Debug)]
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
        let valid_moves = if let Some(pos) = self.selected_position {
            match self.cells[pos] {
                Piece::Tiger => self.get_valid_tiger_moves(pos),
                Piece::Goat => self.get_valid_goat_moves(pos),
                Piece::Empty => vec![],
            }
        } else {
            vec![]
        };

        let mut display = String::new();
        for (i, cell) in self.cells.iter().enumerate() {
            if i % 5 == 0 {
                display.push_str("   ");
            }

            let piece = match cell {
                Piece::Tiger => {
                    if Some(i) == self.selected_position {
                        "T".red().bold().on_bright_black().to_string()
                    } else {
                        "T".red().bold().to_string()
                    }
                }
                Piece::Goat => {
                    if Some(i) == self.selected_position {
                        "G".yellow().bold().on_bright_black().to_string()
                    } else {
                        "G".yellow().bold().to_string()
                    }
                }
                Piece::Empty => {
                    if valid_moves.contains(&Position(i)) {
                        "•".bright_green().bold().to_string()
                    } else {
                        "·".to_string()
                    }
                }
            };
            display.push_str(&piece);

            if (i + 1) % 5 == 0 {
                display.push('\n');
            } else {
                display.push(' ');
            }
        }
        display
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
        // In a 5x5 grid, diagonal moves are allowed at these positions:
        const DIAGONAL_POSITIONS: [usize; 13] = [
            0, 2, 4, // Top row
            6, 8, // Second row
            10, 12, 14, // Middle row
            16, 18, // Fourth row
            20, 22, 24, // Bottom row
        ];
        DIAGONAL_POSITIONS.contains(&pos)
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

    pub fn ai_move_tiger(&mut self) -> bool {
        let moves = self.get_all_valid_tiger_moves();
        if moves.is_empty() {
            return false;
        }

        // First priority: Look for capture moves
        let capture_moves: Vec<_> = moves
            .iter()
            .filter(|&(from, to)| self.get_captured_position(*from, *to).is_some())
            .collect();

        if !capture_moves.is_empty() {
            // Choose a random capture move
            use rand::seq::SliceRandom;
            if let Some(&(from, to)) = capture_moves.choose(&mut rand::thread_rng()) {
                return self.move_tiger(*from, *to);
            }
        }

        // Second priority: Look for moves that could set up future captures
        let strategic_moves: Vec<_> = moves
            .iter()
            .filter(|&(from, to)| {
                // Check if this move puts us next to a goat
                let to_row = *to / 5;
                let to_col = *to % 5;

                for &(dr, dc) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let new_row = to_row as i32 + dr;
                    let new_col = to_col as i32 + dc;

                    if new_row >= 0 && new_row < 5 && new_col >= 0 && new_col < 5 {
                        let pos = (new_row * 5 + new_col) as usize;
                        if self.cells[pos] == Piece::Goat {
                            return true;
                        }
                    }
                }
                false
            })
            .collect();

        if !strategic_moves.is_empty() {
            use rand::seq::SliceRandom;
            if let Some(&(from, to)) = strategic_moves.choose(&mut rand::thread_rng()) {
                return self.move_tiger(*from, *to);
            }
        }

        // Last resort: Make a random move
        use rand::seq::SliceRandom;
        if let Some(&(from, to)) = moves.choose(&mut rand::thread_rng()) {
            return self.move_tiger(from, to);
        }

        false
    }

    pub fn ai_move_goat(&mut self) -> bool {
        if self.goats_in_hand > 0 {
            return self.ai_place_goat();
        }

        // Get all possible moves for existing goats
        let moves: Vec<(usize, usize)> = (0..25)
            .filter(|&pos| self.cells[pos] == Piece::Goat)
            .flat_map(|pos| {
                self.get_valid_goat_moves(pos)
                    .into_iter()
                    .map(move |to| (pos, to.0))
            })
            .collect();

        if moves.is_empty() {
            return false;
        }

        // First priority: Look for moves that trap tigers
        let trapping_moves: Vec<_> = moves
            .iter()
            .filter(|&(from, to)| {
                // Temporarily make the move
                let original_from = self.cells[*from];
                let original_to = self.cells[*to];
                self.cells[*from] = Piece::Empty;
                self.cells[*to] = Piece::Goat;

                // Check if this creates a trap
                let tigers_can_move = (0..25)
                    .filter(|&pos| self.cells[pos] == Piece::Tiger)
                    .any(|pos| !self.get_valid_tiger_moves(pos).is_empty());

                // Undo the temporary move
                self.cells[*from] = original_from;
                self.cells[*to] = original_to;

                !tigers_can_move
            })
            .collect();

        if !trapping_moves.is_empty() {
            use rand::seq::SliceRandom;
            if let Some(&(from, to)) = trapping_moves.choose(&mut rand::thread_rng()) {
                return self.move_goat(*from, *to);
            }
        }

        // Second priority: Avoid moves that could lead to capture
        let safe_moves: Vec<_> = moves
            .iter()
            .filter(|&(_from, to)| {
                // Check if this move would put us in danger of capture
                let to_row = *to / 5;
                let to_col = *to % 5;

                // Check if there's a tiger that could capture us
                for &(dr, dc) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let new_row = to_row as i32 + dr;
                    let new_col = to_col as i32 + dc;

                    if new_row >= 0 && new_row < 5 && new_col >= 0 && new_col < 5 {
                        let pos = (new_row * 5 + new_col) as usize;
                        if self.cells[pos] == Piece::Tiger {
                            // Check if there's an empty space for the tiger to jump to
                            let jump_row = new_row + dr;
                            let jump_col = new_col + dc;
                            if jump_row >= 0 && jump_row < 5 && jump_col >= 0 && jump_col < 5 {
                                let jump_pos = (jump_row * 5 + jump_col) as usize;
                                if self.cells[jump_pos] == Piece::Empty {
                                    return false;
                                }
                            }
                        }
                    }
                }
                true
            })
            .collect();

        if !safe_moves.is_empty() {
            use rand::seq::SliceRandom;
            if let Some(&(from, to)) = safe_moves.choose(&mut rand::thread_rng()) {
                return self.move_goat(*from, *to);
            }
        }

        // Last resort: Make a random move
        use rand::seq::SliceRandom;
        if let Some(&(from, to)) = moves.choose(&mut rand::thread_rng()) {
            return self.move_goat(from, to);
        }

        false
    }

    fn ai_place_goat(&mut self) -> bool {
        if self.goats_in_hand == 0 {
            return false;
        }

        // Priority positions for goat placement
        let priority_positions = [
            12, // Center
            6, 8, 16, 18, // Diagonal positions
            7, 11, 13, 17, // Adjacent to center
        ];

        // Try priority positions first
        for &pos in &priority_positions {
            if self.cells[pos] == Piece::Empty {
                return self.place_goat(pos);
            }
        }

        // If no priority positions are available, find all valid positions
        let valid_positions: Vec<usize> = (0..25)
            .filter(|&pos| self.cells[pos] == Piece::Empty)
            .collect();

        if !valid_positions.is_empty() {
            use rand::seq::SliceRandom;
            if let Some(&pos) = valid_positions.choose(&mut rand::thread_rng()) {
                return self.place_goat(pos);
            }
        }

        false
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
