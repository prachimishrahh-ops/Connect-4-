# Connect4 Game Logic - Usage Guide

This document explains how to use the production-ready Connect4 game logic implemented in `abi/src/connect4.rs`.

## Quick Start

```rust
use abi::connect4::{Connect4Game, Player, drop_disc, check_winner, is_board_full};

// Create a new game
let mut game = Connect4Game::new("game-123".to_string());
game.start();

// Make moves
if let Some(row) = drop_disc(&mut game.board, 3, Player::Red) {
    if check_winner(&game.board, row, 3) {
        game.winner = Some(Player::Red);
        game.status = GameStatus::Finished;
    } else if is_board_full(&game.board) {
        game.status = GameStatus::Draw;
    } else {
        game.current_turn = game.current_turn.opponent();
    }
}
```

## Core Types

### Player
```rust
pub enum Player {
    Red,    // Goes first
    Yellow, // Goes second
}

// Helper methods
player.opponent()  // Returns the other player
player.as_str()    // Returns "Red" or "Yellow"
```

### GameStatus
```rust
pub enum GameStatus {
    WaitingForPlayers,  // Initial state
    InProgress,         // Game is active
    Finished,          // Game ended with winner
    Draw,              // Board full, no winner
}
```

### Move
```rust
pub struct Move {
    pub player: Player,
    pub column: u8,     // 0-6
    pub row: u8,        // Where it landed (0-5)
    pub timestamp: u64,
}
```

### Connect4Game
```rust
pub struct Connect4Game {
    pub game_id: String,
    pub board: Board,                  // 6x7 array
    pub current_turn: Player,
    pub move_history: Vec<Move>,
    pub status: GameStatus,
    pub winner: Option<Player>,
}
```

## Core Functions

### drop_disc
Drops a disc into a column with gravity simulation.

```rust
pub fn drop_disc(
    board: &mut Board,
    column: usize,
    player: Player
) -> Option<usize>
```

**Returns:**
- `Some(row)` - The row where the disc landed (0-5)
- `None` - If column is full or invalid

**Example:**
```rust
let mut game = Connect4Game::new("game-1".to_string());

match drop_disc(&mut game.board, 3, Player::Red) {
    Some(row) => {
        println!("Disc landed at row {}", row);
        // Store move in history
        game.move_history.push(Move {
            player: Player::Red,
            column: 3,
            row: row as u8,
            timestamp: get_current_time(),
        });
    }
    None => {
        println!("Column 3 is full!");
    }
}
```

### check_winner
Checks if the last move resulted in a win.

```rust
pub fn check_winner(
    board: &Board,
    last_row: usize,
    last_col: usize
) -> bool
```

**Checks all 4 directions:**
- Horizontal (←→)
- Vertical (↑↓)
- Diagonal descending (↘↖)
- Diagonal ascending (↗↙)

**Example:**
```rust
if let Some(row) = drop_disc(&mut game.board, 3, Player::Red) {
    if check_winner(&game.board, row, 3) {
        game.winner = Some(Player::Red);
        game.status = GameStatus::Finished;
        println!("Player Red wins!");
    }
}
```

### is_board_full
Checks if the board is completely full (draw condition).

```rust
pub fn is_board_full(board: &Board) -> bool
```

**Example:**
```rust
if is_board_full(&game.board) && game.winner.is_none() {
    game.status = GameStatus::Draw;
    println!("Game ended in a draw!");
}
```

### is_column_available
Checks if a column has space for another disc.

```rust
pub fn is_column_available(board: &Board, column: usize) -> bool
```

**Example:**
```rust
if is_column_available(&game.board, 3) {
    // Column 3 has space
    drop_disc(&mut game.board, 3, Player::Red);
} else {
    println!("Column 3 is full!");
}
```

### get_available_columns
Returns all columns that have space.

```rust
pub fn get_available_columns(board: &Board) -> Vec<usize>
```

**Example:**
```rust
let available = get_available_columns(&game.board);
println!("Available columns: {:?}", available); // [0, 1, 2, 4, 5, 6]
```

## Complete Game Loop Example

```rust
use abi::connect4::*;

fn play_game() {
    // Create game
    let mut game = Connect4Game::new("match-001".to_string());
    game.start();

    // Game loop
    loop {
        // Get player move (from blockchain transaction, user input, etc.)
        let column = get_player_move(game.current_turn);

        // Validate column is available
        if !is_column_available(&game.board, column) {
            println!("Invalid move: column {} is full", column);
            continue;
        }

        // Drop the disc
        let row = drop_disc(&mut game.board, column, game.current_turn)
            .expect("Column should be available");

        // Record move in history
        game.move_history.push(Move {
            player: game.current_turn,
            column: column as u8,
            row: row as u8,
            timestamp: get_timestamp(),
        });

        // Check for winner
        if check_winner(&game.board, row, column) {
            game.winner = Some(game.current_turn);
            game.status = GameStatus::Finished;
            println!("{} wins!", game.current_turn.as_str());
            break;
        }

        // Check for draw
        if is_board_full(&game.board) {
            game.status = GameStatus::Draw;
            println!("Game ended in a draw!");
            break;
        }

        // Next player's turn
        game.current_turn = game.current_turn.opponent();
    }
}
```

## Integration with Linera Blockchain

### State Storage
```rust
use linera_sdk::views::{RootView, View};

#[derive(RootView)]
pub struct GameState {
    pub games: MapView<String, Connect4Game>,
}
```

### Message Handling
```rust
use linera_sdk::{Contract, ContractRuntime};

impl Contract for GameContract {
    fn handle_operation(&mut self, operation: Operation) -> Result<Response> {
        match operation {
            Operation::MakeMove { game_id, column } => {
                let mut game = self.state.games.get_mut(&game_id)?;

                // Validate it's the current player's turn
                require!(
                    game.current_turn == get_current_chain(),
                    "Not your turn"
                );

                // Make the move
                let row = drop_disc(&mut game.board, column, game.current_turn)
                    .ok_or("Column is full")?;

                // Record move
                game.move_history.push(Move {
                    player: game.current_turn,
                    column: column as u8,
                    row: row as u8,
                    timestamp: runtime.system_time().micros(),
                });

                // Check game end conditions
                if check_winner(&game.board, row, column) {
                    game.winner = Some(game.current_turn);
                    game.status = GameStatus::Finished;
                } else if is_board_full(&game.board) {
                    game.status = GameStatus::Draw;
                } else {
                    game.current_turn = game.current_turn.opponent();
                }

                Ok(Response::MoveAccepted)
            }
        }
    }
}
```

## Performance Characteristics

- **Board Storage**: 336 bytes (6 rows × 7 columns × 8 bytes per Option<Player>)
- **drop_disc**: O(ROWS) = O(6) = Constant time
- **check_winner**: O(WIN_LENGTH) = O(4) = Constant time
- **is_board_full**: O(COLS) = O(7) = Constant time
- **get_available_columns**: O(COLS) = O(7) = Constant time

All operations are constant time relative to the fixed board size.

## Security Considerations

### Input Validation
```rust
// ALWAYS validate column index
if column >= COLS {
    return Err("Invalid column");
}

// ALWAYS check column availability
if !is_column_available(&game.board, column) {
    return Err("Column is full");
}

// ALWAYS verify it's the correct player's turn
if game.current_turn != current_player {
    return Err("Not your turn");
}
```

### State Verification
```rust
// Verify game is in progress
if game.status != GameStatus::InProgress {
    return Err("Game is not in progress");
}

// Prevent replay attacks by checking move history
if game.move_history.len() >= ROWS * COLS {
    return Err("Game should have ended");
}
```

## Testing

Run the comprehensive test suite:

```bash
cd abi
cargo test --lib connect4
```

All 27 tests cover:
- ✅ All win directions (horizontal, vertical, both diagonals)
- ✅ Edge cases (full column, draw, boundaries)
- ✅ Player alternation
- ✅ Move validation
- ✅ Board state management

## Board Coordinates

```
   0   1   2   3   4   5   6  (columns)
0 [ ] [ ] [ ] [ ] [ ] [ ] [ ]  ← Top row
1 [ ] [ ] [ ] [ ] [ ] [ ] [ ]
2 [ ] [ ] [ ] [ ] [ ] [ ] [ ]
3 [ ] [ ] [ ] [ ] [ ] [ ] [ ]
4 [ ] [ ] [ ] [ ] [ ] [ ] [ ]
5 [ ] [ ] [ ] [ ] [ ] [ ] [ ]  ← Bottom row
```

- **Columns**: 0-6 (left to right)
- **Rows**: 0-5 (top to bottom)
- Discs fall to the lowest available row in each column

## Constants

```rust
pub const ROWS: usize = 6;        // Standard Connect4 height
pub const COLS: usize = 7;        // Standard Connect4 width
pub const WIN_LENGTH: usize = 4;  // 4 in a row to win
```

## Error Handling

All functions use `Option<T>` for safe error handling:

```rust
match drop_disc(&mut game.board, column, player) {
    Some(row) => {
        // Success - disc landed at row
    }
    None => {
        // Error - column was full or invalid
    }
}
```

No panics, no unwraps in production code. All edge cases handled gracefully.
