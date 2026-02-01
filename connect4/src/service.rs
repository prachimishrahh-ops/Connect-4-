#![cfg_attr(target_arch = "wasm32", no_main)]

//! Connect4 Battle Service
//!
//! GraphQL query service for the Connect4 game on Linera blockchain.

mod state;

use std::sync::Arc;

use abi::connect4::Player;
use abi::leaderboard::SimpleLeaderboardEntry;
use abi::player::PlayerProfile;
use async_graphql::{EmptySubscription, Object, Schema};
use connect4::Connect4Operation;
use linera_sdk::linera_base_types::ChainId;
use linera_sdk::{
    graphql::GraphQLMutationRoot, linera_base_types::WithServiceAbi, views::View, Service,
    ServiceRuntime,
};

use self::state::{Connect4GameState, Connect4State};

pub struct Connect4Service {
    state: Arc<Connect4State>,
    runtime: Arc<ServiceRuntime<Self>>,
}

linera_sdk::service!(Connect4Service);

impl WithServiceAbi for Connect4Service {
    type Abi = connect4::Connect4Abi;
}

impl Service for Connect4Service {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = Connect4State::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        Connect4Service {
            state: Arc::new(state),
            runtime: Arc::new(runtime),
        }
    }

    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
        Schema::build(
            QueryRoot {
                state: self.state.clone(),
                runtime: self.runtime.clone(),
            },
            Connect4Operation::mutation_root(self.runtime.clone()),
            EmptySubscription,
        )
        .finish()
        .execute(query)
        .await
    }
}

#[allow(dead_code)]
struct QueryRoot {
    state: Arc<Connect4State>,
    runtime: Arc<ServiceRuntime<Connect4Service>>,
}

#[Object]
impl QueryRoot {
    /// Get the chain type (0=Master, 1=Lobby, 2=Game, 3=User)
    async fn get_chain_type(&self) -> u64 {
        *self.state.chain_type.get()
    }

    // ============================================
    // USER CHAIN QUERIES
    // ============================================

    /// Get the user's profile
    async fn get_user_profile(&self) -> Option<PlayerProfile> {
        self.state.user_profile.get().clone()
    }

    /// Get the user's assigned color in current game
    async fn get_user_color(&self) -> Option<String> {
        self.state.user_color.get().map(|p| match p {
            Player::Red => "Red".to_string(),
            Player::Yellow => "Yellow".to_string(),
        })
    }

    /// Get the current game state (from subscription)
    async fn get_game_state(&self) -> Option<Connect4GameStateView> {
        self.state.channel_game_state.get().clone().map(|g| g.into())
    }

    /// Get the lobby chain ID
    async fn get_lobby_chain(&self) -> Option<ChainId> {
        *self.state.lobby_chain.get()
    }

    /// Get the user's current game chain
    async fn get_user_game_chain(&self) -> Option<ChainId> {
        *self.state.user_game_chain.get()
    }

    // ============================================
    // LOBBY CHAIN QUERIES
    // ============================================

    /// Get the number of players in matchmaking queue
    async fn get_queue_count(&self) -> u32 {
        *self.state.queue_count.get()
    }

    // ============================================
    // GAME CHAIN QUERIES
    // ============================================

    /// Get the current game on this game chain
    async fn get_current_game(&self) -> Option<Connect4GameStateView> {
        self.state.current_game.get().clone().map(|g| g.into())
    }

    /// Check if this game chain is available
    async fn is_game_chain_available(&self) -> bool {
        *self.state.game_chain_available.get()
    }

    /// Get total games hosted on this chain
    async fn get_games_hosted(&self) -> u64 {
        *self.state.games_hosted.get()
    }

    // ============================================
    // MASTER CHAIN QUERIES
    // ============================================

    /// Get all leaderboard entries
    async fn get_leaderboard(&self) -> Vec<SimpleLeaderboardEntry> {
        let keys = self
            .state
            .leaderboard
            .indices()
            .await
            .expect("Failed to get leaderboard keys");

        let mut entries = Vec::new();
        for key in keys {
            if let Some(entry) = self
                .state
                .leaderboard
                .get(&key)
                .await
                .expect("Failed to get leaderboard entry")
            {
                entries.push(entry);
            }
        }

        // Sort by ELO (highest first)
        entries.sort_by(|a, b| b.elo.cmp(&a.elo));
        entries
    }

    /// Get registered player count
    async fn get_registered_player_count(&self) -> u64 {
        self.state
            .registered_players
            .indices()
            .await
            .expect("Failed to count players")
            .len() as u64
    }
}

/// GraphQL-friendly view of Connect4 game state
#[derive(async_graphql::SimpleObject)]
struct Connect4GameStateView {
    /// Game ID
    pub game_id: u64,
    /// Board as a flat array (row-major, 6 rows x 7 columns = 42 cells)
    /// Each cell is: null (empty), "Red", or "Yellow"
    pub board: Vec<Option<String>>,
    /// Current turn
    pub current_turn: String,
    /// Game status
    pub status: String,
    /// Winner if finished
    pub winner: Option<String>,
    /// Number of moves made
    pub move_count: u32,
    /// Red player chain ID
    pub red_player_chain: Option<ChainId>,
    /// Red player name
    pub red_player_name: Option<String>,
    /// Yellow player chain ID
    pub yellow_player_chain: Option<ChainId>,
    /// Yellow player name
    pub yellow_player_name: Option<String>,
}

impl From<Connect4GameState> for Connect4GameStateView {
    fn from(game: Connect4GameState) -> Self {
        // Flatten board to array
        let board: Vec<Option<String>> = game.board
            .iter()
            .flat_map(|row| {
                row.iter().map(|cell| {
                    cell.map(|p| match p {
                        Player::Red => "Red".to_string(),
                        Player::Yellow => "Yellow".to_string(),
                    })
                })
            })
            .collect();

        let current_turn = match game.current_turn {
            Player::Red => "Red".to_string(),
            Player::Yellow => "Yellow".to_string(),
        };

        let status = format!("{:?}", game.status);

        let winner = game.winner.map(|p| match p {
            Player::Red => "Red".to_string(),
            Player::Yellow => "Yellow".to_string(),
        });

        Connect4GameStateView {
            game_id: game.game_id,
            board,
            current_turn,
            status,
            winner,
            move_count: game.move_history.len() as u32,
            red_player_chain: game.red_player.as_ref().map(|p| p.chain_id),
            red_player_name: game.red_player.as_ref().map(|p| p.name.clone()),
            yellow_player_chain: game.yellow_player.as_ref().map(|p| p.chain_id),
            yellow_player_name: game.yellow_player.as_ref().map(|p| p.name.clone()),
        }
    }
}
