// Connect4 Battle state management
// Multi-chain state: Master (0), Lobby (1), Game (2), User (3)

use abi::connect4::{Board, GameStatus, Move, Player};
use abi::game::GameId;
use abi::leaderboard::SimpleLeaderboardEntry;
use abi::management::{GameChainInfo, LobbyChainInfo};
use abi::player::{PlayerProfile, QueuedPlayer};
use bankroll::BankrollAbi;
use linera_sdk::linera_base_types::{Amount, ApplicationId, ChainId, Timestamp};
use linera_sdk::views::{linera_views, MapView, QueueView, RegisterView, RootView, ViewStorageContext};
use serde::{Deserialize, Serialize};

/// Active Connect4 game state stored on Game Chain
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Connect4GameState {
    /// Unique game identifier
    pub game_id: GameId,
    /// The 6x7 game board
    pub board: Board,
    /// Current player's turn (Red or Yellow)
    pub current_turn: Player,
    /// Complete move history
    pub move_history: Vec<Move>,
    /// Current game status
    pub status: GameStatus,
    /// Winner of the game (if finished)
    pub winner: Option<Player>,
    /// Red player info
    pub red_player: Option<Connect4Player>,
    /// Yellow player info
    pub yellow_player: Option<Connect4Player>,
    /// When the game started
    pub started_at: Option<Timestamp>,
    /// When the game ended
    pub ended_at: Option<Timestamp>,
}

impl Default for Connect4GameState {
    fn default() -> Self {
        Self {
            game_id: 0,
            board: [[None; 7]; 6],
            current_turn: Player::Red,
            move_history: Vec::new(),
            status: GameStatus::WaitingForPlayers,
            winner: None,
            red_player: None,
            yellow_player: None,
            started_at: None,
            ended_at: None,
        }
    }
}

/// Player information for Connect4
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Connect4Player {
    /// Player's chain ID
    pub chain_id: ChainId,
    /// Player's display name
    pub name: String,
    /// Player's ELO rating
    pub elo: u32,
    /// Has this player joined the game
    pub joined: bool,
}

#[allow(dead_code)]
impl Connect4Player {
    pub fn new(chain_id: ChainId, name: String, elo: u32) -> Self {
        Connect4Player {
            chain_id,
            name,
            elo,
            joined: false,
        }
    }
}

#[allow(dead_code)]
impl Connect4GameState {
    /// Create a new Connect4 game
    pub fn new(game_id: GameId) -> Self {
        Connect4GameState {
            game_id,
            board: [[None; 7]; 6],
            current_turn: Player::Red, // Red always starts
            move_history: Vec::new(),
            status: GameStatus::WaitingForPlayers,
            winner: None,
            red_player: None,
            yellow_player: None,
            started_at: None,
            ended_at: None,
        }
    }

    /// Check if both players have joined
    pub fn both_players_joined(&self) -> bool {
        self.red_player.as_ref().map(|p| p.joined).unwrap_or(false)
            && self.yellow_player.as_ref().map(|p| p.joined).unwrap_or(false)
    }

    /// Start the game
    pub fn start(&mut self, timestamp: Timestamp) {
        if self.status == GameStatus::WaitingForPlayers && self.both_players_joined() {
            self.status = GameStatus::InProgress;
            self.started_at = Some(timestamp);
        }
    }

    /// Get player by chain ID
    pub fn get_player_by_chain(&self, chain_id: &ChainId) -> Option<(&Connect4Player, Player)> {
        if let Some(ref red) = self.red_player {
            if red.chain_id == *chain_id {
                return Some((red, Player::Red));
            }
        }
        if let Some(ref yellow) = self.yellow_player {
            if yellow.chain_id == *chain_id {
                return Some((yellow, Player::Yellow));
            }
        }
        None
    }

    /// Get mutable player by chain ID
    pub fn get_player_mut_by_chain(&mut self, chain_id: &ChainId) -> Option<(&mut Connect4Player, Player)> {
        if let Some(ref mut red) = self.red_player {
            if red.chain_id == *chain_id {
                return Some((red, Player::Red));
            }
        }
        if let Some(ref mut yellow) = self.yellow_player {
            if yellow.chain_id == *chain_id {
                return Some((yellow, Player::Yellow));
            }
        }
        None
    }

    /// Check if it's the specified player's turn
    pub fn is_player_turn(&self, player_color: Player) -> bool {
        self.current_turn == player_color
    }

    /// Switch to the next player's turn
    pub fn switch_turn(&mut self) {
        self.current_turn = self.current_turn.opponent();
    }

    /// Get the opponent's chain ID
    #[allow(dead_code)]
    pub fn get_opponent_chain(&self, player_chain: &ChainId) -> Option<ChainId> {
        if let Some(ref red) = self.red_player {
            if red.chain_id == *player_chain {
                return self.yellow_player.as_ref().map(|p| p.chain_id);
            }
        }
        if let Some(ref yellow) = self.yellow_player {
            if yellow.chain_id == *player_chain {
                return self.red_player.as_ref().map(|p| p.chain_id);
            }
        }
        None
    }

    /// Get both player chain IDs
    #[allow(dead_code)]
    pub fn get_player_chains(&self) -> Vec<ChainId> {
        let mut chains = Vec::new();
        if let Some(ref red) = self.red_player {
            chains.push(red.chain_id);
        }
        if let Some(ref yellow) = self.yellow_player {
            chains.push(yellow.chain_id);
        }
        chains
    }
}

#[derive(RootView)]
#[view(context = ViewStorageContext)]
pub struct LiarsDiceState {
    // ============================================
    // ALL CHAINS - Common state
    // ============================================
    /// Current chain type (set during instantiation)
    pub chain_type: RegisterView<u64>,

    // ============================================
    // MASTER CHAIN STATE (instantiate_value = 0)
    // ============================================
    /// Registered lobby chains
    pub lobby_chains: MapView<ChainId, LobbyChainInfo>,
    /// Global leaderboard entries
    pub leaderboard: MapView<ChainId, SimpleLeaderboardEntry>,
    /// All registered player profiles (for global lookups)
    pub registered_players: MapView<ChainId, PlayerProfile>,

    // ============================================
    // LOBBY CHAIN STATE (instantiate_value = 1)
    // ============================================
    /// Matchmaking queue
    pub matchmaking_queue: QueueView<QueuedPlayer>,
    /// Available game chains pool
    pub available_game_chains: QueueView<ChainId>,
    /// Game chains currently in use
    pub active_game_chains: MapView<ChainId, GameChainInfo>,
    /// Queue count for quick access
    pub queue_count: RegisterView<u32>,

    // ============================================
    // GAME CHAIN STATE (instantiate_value = 2)
    // ============================================
    /// Current active Connect4 game on this chain
    pub current_game: RegisterView<Option<Connect4GameState>>,
    /// Is this game chain available?
    pub game_chain_available: RegisterView<bool>,
    /// Total games hosted on this chain
    pub games_hosted: RegisterView<u64>,

    // ============================================
    // USER CHAIN STATE (instantiate_value = 3)
    // ============================================
    /// User's profile
    pub user_profile: RegisterView<Option<PlayerProfile>>,
    /// Current game chain user is connected to
    pub user_game_chain: RegisterView<Option<ChainId>>,
    /// Current lobby chain
    pub lobby_chain: RegisterView<Option<ChainId>>,
    /// User's token balance (cached from bankroll)
    pub user_balance: RegisterView<Amount>,
    /// User's assigned color in current game
    pub user_color: RegisterView<Option<Player>>,
    /// Last received game state (from event subscription)
    pub channel_game_state: RegisterView<Option<Connect4GameState>>,

    // ============================================
    // PARAMETERS (ALL CHAINS) - Cached to avoid runtime.application_parameters() in Linera 0.15.7
    // ============================================
    /// Cached master chain ID
    pub cached_master_chain: RegisterView<Option<ChainId>>,
    /// Cached bankroll application ID
    pub cached_bankroll: RegisterView<Option<ApplicationId<BankrollAbi>>>,
    /// Cached lobby chain ID
    pub cached_lobby_chain: RegisterView<Option<ChainId>>,
}
