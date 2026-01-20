//! Connect4 Battle - Main Game Contract ABI
//!
//! This module defines the Application Binary Interface (ABI) for the Connect4 Battle game
//! running on the Linera blockchain. It implements a 4-chain architecture:
//!
//! - **Master Chain (instantiate_value = 0)**: Admin operations, global state, leaderboard
//! - **Lobby Chain (instantiate_value = 1)**: Matchmaking queue, player pairing
//! - **Game Chain (instantiate_value = 2)**: Individual game sessions, move validation
//! - **User Chain (instantiate_value = 3)**: Player profiles, game participation
//!
//! # Architecture Overview
//!
//! ```text
//! User Chain <---> Lobby Chain <---> Game Chain
//!      |               |                  |
//!      +---------------+------------------+
//!                      |
//!                Master Chain
//! ```

use abi::connect4::{Board, GameStatus, Move, Player};
use abi::game::GameId;
use abi::leaderboard::SimpleLeaderboardEntry;
use abi::player::{PlayerProfile, QueuedPlayer};
use async_graphql::{Request, Response};
use bankroll::BankrollAbi;
use linera_sdk::{
    graphql::GraphQLMutationRoot,
    linera_base_types::{Amount, ApplicationId, ChainId, ContractAbi, ServiceAbi},
};
use serde::{Deserialize, Serialize};

/// Stream name for Connect4 game events.
///
/// Used for real-time event subscriptions across chains.
pub const CONNECT4_STREAM_NAME: &[u8] = b"connect4";

/// Connect4 Battle Application Binary Interface.
///
/// Defines the contract and service interfaces for cross-chain communication.
#[derive(Debug, Deserialize, Serialize)]
pub struct Connect4Abi;

impl ContractAbi for Connect4Abi {
    type Operation = Connect4Operation;
    type Response = ();
}

impl ServiceAbi for Connect4Abi {
    type Query = Request;
    type QueryResponse = Response;
}

/// Operations that can be called on the Connect4 contract.
///
/// Operations are grouped by the chain type that should handle them:
/// - User chain operations are initiated by players
/// - Master chain operations are for admin/system management
#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Connect4Operation {
    // ============================================
    // USER CHAIN OPERATIONS (instantiate_value = 3)
    // ============================================
    /// Set or update the player's display name.
    ///
    /// # Parameters
    /// - `name`: The display name (1-32 characters, alphanumeric and spaces)
    SetProfile {
        /// Player's display name
        name: String,
    },

    /// Request to find a match through the lobby.
    ///
    /// Sends the player to the matchmaking queue where they will be paired
    /// with another player of similar ELO rating.
    FindMatch {},

    /// Cancel an active matchmaking request.
    ///
    /// Removes the player from the matchmaking queue if they haven't been
    /// matched yet.
    CancelMatch {},

    /// Make a move in the current game by dropping a disc into a column.
    ///
    /// # Parameters
    /// - `column`: Column index (0-6) where the disc should be dropped
    ///
    /// # Errors
    /// Returns an error if:
    /// - It's not the player's turn
    /// - The column is invalid (>6) or full
    /// - The game is not in progress
    MakeMove {
        /// Column to drop disc into (0-6)
        column: u8,
    },

    /// Surrender the current game.
    ///
    /// The opponent wins immediately and ELO ratings are updated accordingly.
    Surrender {},

    /// Exit the current game after it has finished.
    ///
    /// Cleans up local game state and returns the player to idle status.
    ExitGame {},

    /// Query the player's current token balance from the bankroll.
    GetBalance {},

    /// Initial setup for a new user chain.
    ///
    /// Subscribes to the lobby chain for matchmaking events.
    /// Must be called once after the application is instantiated on a user chain.
    InitialSetup {},

    // ============================================
    // MASTER CHAIN OPERATIONS (instantiate_value = 0)
    // ============================================
    /// Register a new lobby chain with the master.
    ///
    /// # Parameters
    /// - `chain_id`: The chain ID of the new lobby
    ///
    /// # Authorization
    /// Only callable by the admin account.
    AddLobbyChain {
        /// Chain ID of the lobby to register
        chain_id: ChainId,
    },

    /// Add a new game chain to the available pool.
    ///
    /// Game chains are allocated to matches by the lobby chain.
    ///
    /// # Parameters
    /// - `chain_id`: The chain ID of the new game chain
    ///
    /// # Authorization
    /// Only callable by the admin account.
    AddGameChain {
        /// Chain ID of the game chain to add
        chain_id: ChainId,
    },

    /// Mint tokens for a specific chain.
    ///
    /// Used to fund game chains or reward players.
    ///
    /// # Parameters
    /// - `chain_id`: The target chain to receive tokens
    /// - `amount`: The amount of tokens to mint
    ///
    /// # Authorization
    /// Only callable by the admin account.
    MintToken {
        /// Target chain for minted tokens
        chain_id: ChainId,
        /// Amount to mint
        amount: Amount,
    },
}

/// Cross-chain messages for Connect4 game communication.
///
/// Messages are the primary mechanism for coordination between chains.
/// Each variant documents its source and destination chain types.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Connect4Message {
    // ============================================
    // SUBSCRIPTION CONTROL (Universal)
    // ============================================
    /// Request to subscribe to events from this chain.
    ///
    /// Used to establish cross-chain event notifications.
    Subscribe,

    /// Request to unsubscribe from events.
    ///
    /// Removes the sender from the notification list.
    Unsubscribe,

    // ============================================
    // TO USER CHAIN (from Lobby/Game Chain)
    // ============================================
    /// Notification that a match has been found.
    ///
    /// Sent from the lobby chain to both matched players.
    MatchFound {
        /// The game chain where the match will be played
        game_chain: ChainId,
        /// Unique identifier for this game session
        game_id: GameId,
        /// Display name of the opponent
        opponent_name: String,
        /// ELO rating of the opponent
        opponent_elo: u32,
        /// The color assigned to this player (Red moves first)
        your_color: Player,
    },

    /// Notification that a move has been made in the game.
    ///
    /// Sent to both players after each valid move.
    MoveMade {
        /// Column where the disc was dropped (0-6)
        column: u8,
        /// Row where the disc landed (0-5, 0 is top)
        row: u8,
        /// The player who made the move
        player: Player,
        /// Whether it's now your turn
        your_turn: bool,
        /// Current state of the board after the move
        board: Board,
    },

    /// Notification that the game has ended.
    ///
    /// Sent to both players with their individual results.
    GameResult {
        /// The winning player's chain ID, None for a draw
        winner: Option<ChainId>,
        /// Your ELO change (positive for gain, negative for loss)
        your_elo_change: i32,
        /// Your new ELO rating after this game
        new_elo: u32,
    },

    /// Confirmation that a profile update was successful.
    ProfileUpdated {
        /// The updated profile
        profile: PlayerProfile,
    },

    /// Information about the lobby chain for initial setup.
    LobbyInfo {
        /// The lobby chain to subscribe to
        lobby_chain: ChainId,
    },

    // ============================================
    // TO LOBBY CHAIN (from User Chain)
    // ============================================
    /// Request to join the matchmaking queue.
    JoinMatchmaking {
        /// The user's chain ID for callbacks
        user_chain: ChainId,
        /// Player's display name
        player_name: String,
        /// Player's current ELO rating
        elo: u32,
    },

    /// Request to leave the matchmaking queue.
    LeaveMatchmaking {
        /// The user chain requesting to leave
        user_chain: ChainId,
    },

    /// Notification that a game has ended.
    ///
    /// Sent from game chain to lobby for cleanup and stats.
    GameEnded {
        /// The unique game identifier
        game_id: GameId,
        /// The winning player's chain ID, None for a draw
        winner: Option<ChainId>,
        /// The Red player's chain ID
        red_player: ChainId,
        /// The Yellow player's chain ID
        yellow_player: ChainId,
    },

    /// Register a game chain with the lobby.
    ///
    /// Sent from master chain when a new game chain is added.
    RegisterGameChain {
        /// The game chain to register
        chain_id: ChainId,
    },

    // ============================================
    // TO GAME CHAIN (from Lobby/User Chain)
    // ============================================
    /// Assign two players to start a new game on this chain.
    ///
    /// Sent from lobby chain when a match is created.
    AssignMatch {
        /// Unique identifier for this game
        game_id: GameId,
        /// First player (will be Red)
        player1: QueuedPlayer,
        /// Second player (will be Yellow)
        player2: QueuedPlayer,
    },

    /// Request to join an assigned game.
    ///
    /// Sent from user chain when player acknowledges the match.
    JoinGame {
        /// The user chain of the joining player
        user_chain: ChainId,
    },

    /// Submit a move in the game.
    ///
    /// Sent from user chain when player makes a move.
    PlayerMove {
        /// The user chain of the player making the move
        user_chain: ChainId,
        /// The column to drop the disc into (0-6)
        column: u8,
    },

    /// Player surrenders the game.
    ///
    /// Sent from user chain when player gives up.
    PlayerSurrender {
        /// The user chain of the surrendering player
        user_chain: ChainId,
    },

    // ============================================
    // TO MASTER CHAIN (from Game/Lobby Chain)
    // ============================================
    /// Request lobby chain information.
    ///
    /// Sent from user chain during initial setup.
    RequestLobbyInfo {
        /// The user chain requesting the info
        user_chain: ChainId,
    },

    /// Update the global leaderboard with game results.
    ///
    /// Sent from game chain when a game concludes.
    UpdateLeaderboard {
        /// Winner's chain ID
        winner: ChainId,
        /// Winner's display name
        winner_name: String,
        /// Winner's new ELO after the match
        winner_new_elo: u32,
        /// Loser's chain ID
        loser: ChainId,
        /// Loser's display name
        loser_name: String,
        /// Loser's new ELO after the match
        loser_new_elo: u32,
    },
}

/// Application initialization parameters.
///
/// These parameters are set when the application is first deployed
/// and cannot be changed afterward.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Connect4Parameters {
    /// The master chain for admin operations and global state.
    pub master_chain: ChainId,
    /// The lobby chain for matchmaking.
    pub lobby_chain: ChainId,
    /// The bankroll application for token management.
    pub bankroll: ApplicationId<BankrollAbi>,
}

/// Events emitted for real-time game updates.
///
/// Events are broadcast to subscribers and can be used for
/// building reactive user interfaces.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Connect4Event {
    /// Complete game state update.
    ///
    /// Sent when significant game state changes occur.
    GameState {
        /// Unique game identifier
        game_id: GameId,
        /// Current board state
        board: Board,
        /// Current game status
        status: GameStatus,
        /// Whose turn it is
        current_turn: Player,
        /// Winner if game is finished
        winner: Option<Player>,
        /// History of all moves
        move_history: Vec<Move>,
    },

    /// Matchmaking queue status update.
    QueueUpdate {
        /// Number of players currently in queue
        players_in_queue: u32,
    },

    /// Global leaderboard update.
    LeaderboardUpdate {
        /// Top leaderboard entries
        entries: Vec<SimpleLeaderboardEntry>,
    },

    /// Player profile update notification.
    ProfileUpdate {
        /// The updated profile
        profile: PlayerProfile,
    },

    /// A move was made in a game.
    MoveUpdate {
        /// The game where the move was made
        game_id: GameId,
        /// The move that was made
        move_made: Move,
        /// Whether this move resulted in a win
        is_winning_move: bool,
    },

    /// A game has started.
    GameStarted {
        /// The game identifier
        game_id: GameId,
        /// Red player's chain ID
        red_player: ChainId,
        /// Red player's display name
        red_name: String,
        /// Yellow player's chain ID
        yellow_player: ChainId,
        /// Yellow player's display name
        yellow_name: String,
    },

    /// A game has ended.
    GameEnded {
        /// The game identifier
        game_id: GameId,
        /// The winning player's chain ID, None for draw
        winner: Option<ChainId>,
        /// How the game ended
        end_reason: GameEndReason,
    },
}

/// Reason for game ending.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum GameEndReason {
    /// A player achieved four in a row
    FourInARow,
    /// The board is full with no winner
    Draw,
    /// A player surrendered
    Surrender,
    /// A player disconnected/timed out
    Timeout,
}

// ============================================
// BACKWARD COMPATIBILITY ALIASES
// ============================================
// These aliases maintain compatibility with the original codebase structure.
// The game logic has been fully converted to Connect4 while preserving
// the proven state management and messaging infrastructure.

/// Backward compatibility alias for Connect4Abi
pub type LiarsDiceAbi = Connect4Abi;

/// Backward compatibility alias for Connect4Operation
pub type LiarsDiceOperation = Connect4Operation;

/// Backward compatibility alias for Connect4Message
pub type LiarsDiceMessage = Connect4Message;

/// Backward compatibility alias for Connect4Parameters
pub type LiarsDiceParameters = Connect4Parameters;

/// Backward compatibility alias for Connect4Event
pub type LiarsDiceEvent = Connect4Event;

/// Backward compatibility alias for CONNECT4_STREAM_NAME
pub const LIARS_DICE_STREAM_NAME: &[u8] = CONNECT4_STREAM_NAME;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect4_abi_serialization() {
        let op = Connect4Operation::MakeMove { column: 3 };
        let serialized = serde_json::to_string(&op).expect("Failed to serialize operation");
        assert!(serialized.contains("MakeMove"));
        assert!(serialized.contains("3"));
    }

    #[test]
    fn test_message_serialization() {
        let msg = Connect4Message::PlayerMove {
            user_chain: ChainId::root(0),
            column: 4,
        };
        let serialized = serde_json::to_string(&msg).expect("Failed to serialize message");
        assert!(serialized.contains("PlayerMove"));
        assert!(serialized.contains("4"));
    }

    #[test]
    fn test_match_found_message() {
        let msg = Connect4Message::MatchFound {
            game_chain: ChainId::root(1),
            game_id: 42,
            opponent_name: "TestPlayer".to_string(),
            opponent_elo: 1200,
            your_color: Player::Red,
        };

        let serialized = serde_json::to_string(&msg).expect("Failed to serialize");
        let deserialized: Connect4Message =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(msg, deserialized);
    }

    #[test]
    fn test_column_validation_range() {
        // Valid columns are 0-6
        for col in 0..=6u8 {
            let op = Connect4Operation::MakeMove { column: col };
            let serialized = serde_json::to_string(&op).expect("Failed to serialize");
            assert!(serialized.contains(&col.to_string()));
        }
    }

    #[test]
    fn test_game_end_reason_serialization() {
        let reasons = [
            GameEndReason::FourInARow,
            GameEndReason::Draw,
            GameEndReason::Surrender,
            GameEndReason::Timeout,
        ];

        for reason in reasons {
            let event = Connect4Event::GameEnded {
                game_id: 1,
                winner: None,
                end_reason: reason,
            };
            let serialized = serde_json::to_string(&event).expect("Failed to serialize");
            let deserialized: Connect4Event =
                serde_json::from_str(&serialized).expect("Failed to deserialize");
            assert_eq!(event, deserialized);
        }
    }

    #[test]
    fn test_board_in_move_made_message() {
        let empty_board: Board = [[None; 7]; 6];
        let msg = Connect4Message::MoveMade {
            column: 3,
            row: 5,
            player: Player::Red,
            your_turn: true,
            board: empty_board,
        };

        let serialized = serde_json::to_string(&msg).expect("Failed to serialize");
        let deserialized: Connect4Message =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(msg, deserialized);
    }
}
