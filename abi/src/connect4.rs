//! Connect4 game logic module
//!
//! This module implements production-ready Connect4 game logic for the Linera blockchain.
//! All functions are deterministic, pure (where possible), and WASM-compatible.

use serde::{Deserialize, Serialize};

/// Number of rows in the Connect4 board
pub const ROWS: usize = 6;

/// Number of columns in the Connect4 board
pub const COLS: usize = 7;

/// Number of consecutive discs required to win
pub const WIN_LENGTH: usize = 4;

/// 2D array representing the game board
/// - None: Empty cell
/// - Some(Player): Cell occupied by a player's disc
pub type Board = [[Option<Player>; COLS]; ROWS];

/// Represents a player in the game
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Hash)]
pub enum Player {
    /// Red player - always goes first
    Red,
    /// Yellow player - goes second
    Yellow,
}

impl Player {
    /// Returns the opponent of the current player
    #[inline]
    pub fn opponent(self) -> Self {
        match self {
            Player::Red => Player::Yellow,
            Player::Yellow => Player::Red,
        }
    }

    /// Returns the player as a string for display
    #[inline]
    pub fn as_str(self) -> &'static str {
        match self {
            Player::Red => "Red",
            Player::Yellow => "Yellow",
        }
    }
}

/// Represents a single move in the game
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Move {
    /// The player who made this move
    pub player: Player,
    /// The column where the disc was dropped (0-6)
    pub column: u8,
    /// The row where the disc landed (0-5)
    pub row: u8,
    /// Unix timestamp when the move was made
    pub timestamp: u64,
}

/// Current status of the game
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameStatus {
    /// Waiting for players to join
    WaitingForPlayers,
    /// Game is actively being played
    InProgress,
    /// Game has finished with a winner
    Finished,
    /// Game ended in a draw (board full, no winner)
    Draw,
}

/// Complete state of a Connect4 game
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Connect4Game {
    /// Unique identifier for this game
    pub game_id: String,
    /// The current state of the board
    pub board: Board,
    /// Which player's turn it is
    pub current_turn: Player,
    /// Complete history of all moves made
    pub move_history: Vec<Move>,
    /// Current game status
    pub status: GameStatus,
    /// Winner of the game (if finished)
    pub winner: Option<Player>,
}

impl Connect4Game {
    /// Creates a new Connect4 game with empty board
    ///
    /// # Arguments
    /// * `game_id` - Unique identifier for the game
    ///
    /// # Returns
    /// A new game instance with Red player going first
    pub fn new(game_id: String) -> Self {
        Self {
            game_id,
            board: [[None; COLS]; ROWS],
            current_turn: Player::Red,
            move_history: Vec::new(),
            status: GameStatus::WaitingForPlayers,
            winner: None,
        }
    }

    /// Starts the game (transitions from WaitingForPlayers to InProgress)
    pub fn start(&mut self) {
        if self.status == GameStatus::WaitingForPlayers {
            self.status = GameStatus::InProgress;
        }
    }

    /// Checks if it's the specified player's turn
    #[inline]
    pub fn is_player_turn(&self, player: Player) -> bool {
        self.current_turn == player
    }

    /// Checks if the game is still in progress
    #[inline]
    pub fn is_active(&self) -> bool {
        self.status == GameStatus::InProgress
    }
}

/// Attempts to drop a disc into the specified column
///
/// # Arguments
/// * `board` - Mutable reference to the game board
/// * `column` - Column index (0-6) where the disc should be dropped
/// * `player` - The player dropping the disc
///
/// # Returns
/// * `Some(row)` - The row index where the disc landed (0-5)
/// * `None` - If the column is full or invalid
///
/// # Safety
/// This function performs bounds checking and will return None for invalid input
pub fn drop_disc(board: &mut Board, column: usize, player: Player) -> Option<usize> {
    // Validate column is within bounds
    if column >= COLS {
        return None;
    }

    // Find the lowest empty row in this column (gravity simulation)
    // We iterate from bottom (ROWS-1) to top (0)
    for row in (0..ROWS).rev() {
        if board[row][column].is_none() {
            board[row][column] = Some(player);
            return Some(row);
        }
    }

    // Column is full
    None
}

/// Checks if there's a winner after the last move
///
/// This function checks all four possible winning directions from the last move position:
/// 1. Horizontal (left-right)
/// 2. Vertical (up-down)
/// 3. Diagonal descending (top-left to bottom-right)
/// 4. Diagonal ascending (bottom-left to top-right)
///
/// # Arguments
/// * `board` - Reference to the current game board
/// * `last_row` - Row index of the last move (0-5)
/// * `last_col` - Column index of the last move (0-6)
///
/// # Returns
/// * `true` - If 4 or more consecutive discs of the same color are found
/// * `false` - Otherwise
///
/// # Panics
/// Never panics - all array accesses are bounds-checked
pub fn check_winner(board: &Board, last_row: usize, last_col: usize) -> bool {
    // Get the player who just moved
    let Some(player) = board[last_row][last_col] else {
        return false; // No disc at this position
    };

    // Define the four direction vectors: (row_delta, col_delta)
    let directions = [
        (0, 1),   // Horizontal: right
        (1, 0),   // Vertical: down
        (1, 1),   // Diagonal descending: down-right
        (1, -1),  // Diagonal ascending: down-left
    ];

    // Check each direction
    for &(dr, dc) in &directions {
        let mut count = 1; // Count the disc we just placed

        // Check in the positive direction
        count += count_consecutive(board, last_row, last_col, dr, dc, player);

        // Check in the negative direction
        count += count_consecutive(board, last_row, last_col, -dr, -dc, player);

        // Check if we found a winning sequence
        if count >= WIN_LENGTH {
            return true;
        }
    }

    false
}

/// Counts consecutive discs of the same player in a given direction
///
/// # Arguments
/// * `board` - Reference to the game board
/// * `start_row` - Starting row position
/// * `start_col` - Starting column position
/// * `row_delta` - Row direction (-1, 0, or 1)
/// * `col_delta` - Column direction (-1, 0, or 1)
/// * `player` - The player we're checking for
///
/// # Returns
/// Number of consecutive discs found in the specified direction (not counting the starting position)
fn count_consecutive(
    board: &Board,
    start_row: usize,
    start_col: usize,
    row_delta: isize,
    col_delta: isize,
    player: Player,
) -> usize {
    let mut count = 0;
    let mut row = start_row as isize;
    let mut col = start_col as isize;

    loop {
        // Move to next position
        row += row_delta;
        col += col_delta;

        // Check bounds
        if row < 0 || row >= ROWS as isize || col < 0 || col >= COLS as isize {
            break;
        }

        // Check if the cell contains the same player's disc
        if board[row as usize][col as usize] == Some(player) {
            count += 1;
        } else {
            break;
        }
    }

    count
}

/// Checks if the board is completely full (draw condition)
///
/// # Arguments
/// * `board` - Reference to the game board
///
/// # Returns
/// * `true` - If all cells in the top row are occupied (board is full)
/// * `false` - If at least one cell in the top row is empty
///
/// # Note
/// We only need to check the top row because discs fall down due to gravity.
/// If the top row is full, the entire board is full.
#[inline]
pub fn is_board_full(board: &Board) -> bool {
    // Check if top row (index 0) is completely full
    board[0].iter().all(|cell| cell.is_some())
}

/// Checks if a column has space for another disc
///
/// # Arguments
/// * `board` - Reference to the game board
/// * `column` - Column index to check (0-6)
///
/// # Returns
/// * `true` - If the column has at least one empty cell
/// * `false` - If the column is full or invalid
#[inline]
pub fn is_column_available(board: &Board, column: usize) -> bool {
    if column >= COLS {
        return false;
    }
    // Check if top cell is empty
    board[0][column].is_none()
}

/// Returns a list of available columns (0-6) that are not full
///
/// # Arguments
/// * `board` - Reference to the game board
///
/// # Returns
/// Vector of column indices that have space for a disc
pub fn get_available_columns(board: &Board) -> Vec<usize> {
    (0..COLS)
        .filter(|&col| is_column_available(board, col))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to create an empty board
    fn empty_board() -> Board {
        [[None; COLS]; ROWS]
    }

    /// Helper function to create a board with specific discs
    #[allow(dead_code)] // Utility function for future tests
    fn create_board_with_discs(positions: &[(usize, usize, Player)]) -> Board {
        let mut board = empty_board();
        for &(row, col, player) in positions {
            board[row][col] = Some(player);
        }
        board
    }

    #[test]
    fn test_player_opponent() {
        assert_eq!(Player::Red.opponent(), Player::Yellow);
        assert_eq!(Player::Yellow.opponent(), Player::Red);
        assert_eq!(Player::Red.opponent().opponent(), Player::Red);
    }

    #[test]
    fn test_player_as_str() {
        assert_eq!(Player::Red.as_str(), "Red");
        assert_eq!(Player::Yellow.as_str(), "Yellow");
    }

    #[test]
    fn test_new_game() {
        let game = Connect4Game::new("test-game-1".to_string());
        assert_eq!(game.game_id, "test-game-1");
        assert_eq!(game.current_turn, Player::Red);
        assert_eq!(game.status, GameStatus::WaitingForPlayers);
        assert_eq!(game.winner, None);
        assert!(game.move_history.is_empty());
    }

    #[test]
    fn test_game_start() {
        let mut game = Connect4Game::new("test-game-2".to_string());
        game.start();
        assert_eq!(game.status, GameStatus::InProgress);
    }

    #[test]
    fn test_drop_disc_empty_column() {
        let mut board = empty_board();
        let result = drop_disc(&mut board, 3, Player::Red);
        assert_eq!(result, Some(5)); // Should land at bottom row
        assert_eq!(board[5][3], Some(Player::Red));
    }

    #[test]
    fn test_drop_disc_partially_filled_column() {
        let mut board = empty_board();
        drop_disc(&mut board, 2, Player::Red);
        drop_disc(&mut board, 2, Player::Yellow);
        let result = drop_disc(&mut board, 2, Player::Red);

        assert_eq!(result, Some(3)); // Should land at row 3
        assert_eq!(board[5][2], Some(Player::Red));
        assert_eq!(board[4][2], Some(Player::Yellow));
        assert_eq!(board[3][2], Some(Player::Red));
    }

    #[test]
    fn test_drop_disc_full_column() {
        let mut board = empty_board();
        // Fill column 0 completely
        for _ in 0..ROWS {
            drop_disc(&mut board, 0, Player::Red);
        }
        // Try to add one more
        let result = drop_disc(&mut board, 0, Player::Yellow);
        assert_eq!(result, None); // Should fail
    }

    #[test]
    fn test_drop_disc_invalid_column() {
        let mut board = empty_board();
        assert_eq!(drop_disc(&mut board, 7, Player::Red), None);
        assert_eq!(drop_disc(&mut board, 100, Player::Red), None);
    }

    #[test]
    fn test_horizontal_win() {
        let mut board = empty_board();
        // Create horizontal line at bottom row
        board[5][1] = Some(Player::Red);
        board[5][2] = Some(Player::Red);
        board[5][3] = Some(Player::Red);
        board[5][4] = Some(Player::Red);

        assert!(check_winner(&board, 5, 3)); // Check from middle
        assert!(check_winner(&board, 5, 1)); // Check from left end
        assert!(check_winner(&board, 5, 4)); // Check from right end
    }

    #[test]
    fn test_vertical_win() {
        let mut board = empty_board();
        // Create vertical line in column 3
        board[2][3] = Some(Player::Yellow);
        board[3][3] = Some(Player::Yellow);
        board[4][3] = Some(Player::Yellow);
        board[5][3] = Some(Player::Yellow);

        assert!(check_winner(&board, 2, 3)); // Check from top
        assert!(check_winner(&board, 5, 3)); // Check from bottom
    }

    #[test]
    fn test_diagonal_descending_win() {
        let mut board = empty_board();
        // Create diagonal from top-left to bottom-right
        board[2][1] = Some(Player::Red);
        board[3][2] = Some(Player::Red);
        board[4][3] = Some(Player::Red);
        board[5][4] = Some(Player::Red);

        assert!(check_winner(&board, 2, 1)); // Top-left
        assert!(check_winner(&board, 3, 2)); // Middle
        assert!(check_winner(&board, 5, 4)); // Bottom-right
    }

    #[test]
    fn test_diagonal_ascending_win() {
        let mut board = empty_board();
        // Create diagonal from bottom-left to top-right
        board[5][0] = Some(Player::Yellow);
        board[4][1] = Some(Player::Yellow);
        board[3][2] = Some(Player::Yellow);
        board[2][3] = Some(Player::Yellow);

        assert!(check_winner(&board, 5, 0)); // Bottom-left
        assert!(check_winner(&board, 3, 2)); // Middle
        assert!(check_winner(&board, 2, 3)); // Top-right
    }

    #[test]
    fn test_no_winner_three_in_a_row() {
        let mut board = empty_board();
        // Only 3 in a row (not enough to win)
        board[5][2] = Some(Player::Red);
        board[5][3] = Some(Player::Red);
        board[5][4] = Some(Player::Red);

        assert!(!check_winner(&board, 5, 3));
    }

    #[test]
    fn test_no_winner_blocked_sequence() {
        let mut board = empty_board();
        // Sequence blocked by opponent
        board[5][1] = Some(Player::Red);
        board[5][2] = Some(Player::Red);
        board[5][3] = Some(Player::Yellow); // Blocker
        board[5][4] = Some(Player::Red);

        assert!(!check_winner(&board, 5, 1));
        assert!(!check_winner(&board, 5, 4));
    }

    #[test]
    fn test_is_board_full_empty() {
        let board = empty_board();
        assert!(!is_board_full(&board));
    }

    #[test]
    fn test_is_board_full_partial() {
        let mut board = empty_board();
        // Fill bottom row
        for col in 0..COLS {
            board[5][col] = Some(Player::Red);
        }
        assert!(!is_board_full(&board)); // Top row still empty
    }

    #[test]
    fn test_is_board_full_complete() {
        let board = [[Some(Player::Red); COLS]; ROWS];
        assert!(is_board_full(&board));
    }

    #[test]
    fn test_is_column_available() {
        let mut board = empty_board();
        assert!(is_column_available(&board, 0));
        assert!(is_column_available(&board, 6));
        assert!(!is_column_available(&board, 7)); // Invalid column

        // Fill column 3
        for cell in &mut board {
            cell[3] = Some(Player::Red);
        }
        assert!(!is_column_available(&board, 3));
    }

    #[test]
    fn test_get_available_columns_empty_board() {
        let board = empty_board();
        let available = get_available_columns(&board);
        assert_eq!(available, vec![0, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_get_available_columns_some_full() {
        let mut board = empty_board();
        // Fill columns 0 and 6
        for row in &mut board {
            row[0] = Some(Player::Red);
            row[6] = Some(Player::Yellow);
        }
        let available = get_available_columns(&board);
        assert_eq!(available, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_get_available_columns_full_board() {
        let board = [[Some(Player::Red); COLS]; ROWS];
        let available = get_available_columns(&board);
        assert!(available.is_empty());
    }

    #[test]
    fn test_edge_case_win_at_edge() {
        let mut board = empty_board();
        // Win at left edge
        board[5][0] = Some(Player::Red);
        board[5][1] = Some(Player::Red);
        board[5][2] = Some(Player::Red);
        board[5][3] = Some(Player::Red);
        assert!(check_winner(&board, 5, 0));

        let mut board = empty_board();
        // Win at right edge
        board[5][3] = Some(Player::Yellow);
        board[5][4] = Some(Player::Yellow);
        board[5][5] = Some(Player::Yellow);
        board[5][6] = Some(Player::Yellow);
        assert!(check_winner(&board, 5, 6));
    }

    #[test]
    fn test_edge_case_win_at_corners() {
        let mut board = empty_board();
        // Vertical win in leftmost column
        board[0][0] = Some(Player::Red);
        board[1][0] = Some(Player::Red);
        board[2][0] = Some(Player::Red);
        board[3][0] = Some(Player::Red);
        assert!(check_winner(&board, 0, 0));

        let mut board = empty_board();
        // Vertical win in rightmost column
        board[2][6] = Some(Player::Yellow);
        board[3][6] = Some(Player::Yellow);
        board[4][6] = Some(Player::Yellow);
        board[5][6] = Some(Player::Yellow);
        assert!(check_winner(&board, 5, 6));
    }

    #[test]
    fn test_realistic_game_sequence() {
        let mut board = empty_board();

        // Simulate a realistic game
        drop_disc(&mut board, 3, Player::Red);
        assert!(!check_winner(&board, 5, 3));

        drop_disc(&mut board, 3, Player::Yellow);
        assert!(!check_winner(&board, 4, 3));

        drop_disc(&mut board, 4, Player::Red);
        drop_disc(&mut board, 4, Player::Yellow);
        drop_disc(&mut board, 5, Player::Red);
        drop_disc(&mut board, 5, Player::Yellow);
        drop_disc(&mut board, 5, Player::Red);

        // Red wins with diagonal
        let row = drop_disc(&mut board, 6, Player::Red).unwrap();
        assert!(check_winner(&board, row, 6));
    }

    #[test]
    fn test_five_in_a_row_also_wins() {
        let mut board = empty_board();
        // Create 5 in a row (should still be detected as win)
        for col in 0..5 {
            board[5][col] = Some(Player::Red);
        }
        assert!(check_winner(&board, 5, 2)); // Check from middle
    }

    #[test]
    fn test_count_consecutive_boundary() {
        let mut board = empty_board();
        board[5][5] = Some(Player::Red);
        board[5][6] = Some(Player::Red);

        // Should not go out of bounds
        let count = count_consecutive(&board, 5, 5, 0, 1, Player::Red);
        assert_eq!(count, 1); // Only finds the disc at [5][6]
    }

    #[test]
    fn test_different_players_no_win() {
        let mut board = empty_board();
        board[5][2] = Some(Player::Red);
        board[5][3] = Some(Player::Yellow);
        board[5][4] = Some(Player::Red);
        board[5][5] = Some(Player::Red);

        assert!(!check_winner(&board, 5, 2));
        assert!(!check_winner(&board, 5, 5));
    }
}
