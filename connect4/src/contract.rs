#![cfg_attr(target_arch = "wasm32", no_main)]

//! Connect4 Battle Contract
//!
//! A production-ready Connect4 game implementation for the Linera blockchain.
//! Uses a 4-chain architecture:
//! - Master Chain (0): Admin operations, global state, leaderboard
//! - Lobby Chain (1): Matchmaking queue, player pairing
//! - Game Chain (2): Active Connect4 game sessions
//! - User Chain (3): Player profiles, game participation

mod state;

use self::state::{Connect4GameState, Connect4Player, Connect4State};
use abi::connect4::{check_winner, drop_disc, is_board_full, GameStatus, Move, Player};
use abi::leaderboard::SimpleLeaderboardEntry;
use abi::player::{calculate_elo_change, PlayerProfile, QueuedPlayer, UserStatus, STARTING_ELO};
use bankroll::{BankrollAbi, BankrollOperation, BankrollResponse};
use connect4::{
    Connect4Event, Connect4Message, Connect4Operation, Connect4Parameters,
    GameEndReason, CONNECT4_STREAM_NAME,
};
use linera_sdk::linera_base_types::{Amount, ApplicationId, ChainId};
use linera_sdk::{
    linera_base_types::WithContractAbi,
    views::{RootView, View},
    Contract, ContractRuntime,
};

pub struct Connect4Contract {
    state: Connect4State,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(Connect4Contract);

impl WithContractAbi for Connect4Contract {
    type Abi = connect4::Connect4Abi;
}

impl Contract for Connect4Contract {
    type Message = Connect4Message;
    type Parameters = Connect4Parameters;
    type InstantiationArgument = u64; // Chain type: 0=Master, 1=Lobby, 2=Game, 3=User
    type EventValue = Connect4Event;

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = Connect4State::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        Connect4Contract { state, runtime }
    }

    async fn instantiate(&mut self, chain_type: Self::InstantiationArgument) {
        log::info!("Instantiating Connect4 Battle contract with chain_type: {}", chain_type);

        assert!(
            chain_type <= 3,
            "Invalid chain type: {}. Must be 0 (Master), 1 (Lobby), 2 (Game), or 3 (User)",
            chain_type
        );

        self.state.chain_type.set(chain_type);

        match chain_type {
            0 => {
                log::info!("Initialized as MASTER chain (with lobby functionality)");
                // Initialize lobby state on Master chain for matchmaking
                self.state.queue_count.set(0);
            }
            1 => {
                log::info!("Initialized as LOBBY chain");
                self.state.queue_count.set(0);
            }
            2 => {
                log::info!("Initialized as GAME chain");
                self.state.game_chain_available.set(true);
                self.state.games_hosted.set(0);
            }
            3 => {
                log::info!("Initialized as USER chain");
                self.state.user_balance.set(Amount::ZERO);
            }
            _ => unreachable!("Chain type already validated to be 0-3"),
        }
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        let chain_type = *self.state.chain_type.get();

        match operation {
            // ============================================
            // USER CHAIN OPERATIONS
            // ============================================
            Connect4Operation::SetProfile { name } => {
                self.assert_user_chain(chain_type);
                let chain_id = self.runtime.chain_id();
                let owner = self.runtime.authenticated_signer().expect("No authenticated signer");
                let timestamp = self.runtime.system_time();

                let profile = PlayerProfile::new(chain_id, owner, name, timestamp);
                self.state.user_profile.set(Some(profile.clone()));

                log::info!("Profile set for user: {:?}", chain_id);

                self.runtime.emit(
                    CONNECT4_STREAM_NAME.into(),
                    &Connect4Event::ProfileUpdate { profile },
                );
            }

            Connect4Operation::FindMatch {} => {
                self.assert_user_chain(chain_type);
                let profile = self.state.user_profile.get()
                    .as_ref()
                    .expect("Profile not set. Call SetProfile first.");

                let queued_player = QueuedPlayer::new(
                    profile.chain_id.expect("No chain ID"),
                    profile.owner.expect("No owner"),
                    profile.name.clone(),
                    profile.elo,
                    self.runtime.system_time(),
                );

                if let Some(lobby_chain) = self.state.lobby_chain.get().as_ref() {
                    self.message_manager(
                        *lobby_chain,
                        Connect4Message::JoinMatchmaking {
                            user_chain: queued_player.chain_id,
                            player_name: queued_player.name.clone(),
                            elo: queued_player.elo,
                        },
                    );

                    if let Some(ref mut profile) = *self.state.user_profile.get_mut() {
                        profile.set_status(UserStatus::FindingMatch);
                    }

                    log::info!("Player {:?} joining matchmaking queue", queued_player.chain_id);
                } else {
                    log::error!("No lobby chain configured. Call InitialSetup first.");
                }
            }

            Connect4Operation::CancelMatch {} => {
                self.assert_user_chain(chain_type);
                let chain_id = self.runtime.chain_id();

                if let Some(lobby_chain) = self.state.lobby_chain.get().as_ref() {
                    self.message_manager(
                        *lobby_chain,
                        Connect4Message::LeaveMatchmaking { user_chain: chain_id },
                    );

                    if let Some(ref mut profile) = *self.state.user_profile.get_mut() {
                        profile.set_status(UserStatus::Idle);
                    }

                    log::info!("Player {:?} cancelled matchmaking", chain_id);
                }
            }

            Connect4Operation::MakeMove { column } => {
                self.assert_user_chain(chain_type);
                let chain_id = self.runtime.chain_id();

                if column > 6 {
                    log::error!("Invalid column: {}. Must be 0-6.", column);
                    return;
                }

                if let Some(game_chain) = self.state.user_game_chain.get().as_ref() {
                    self.message_manager(
                        *game_chain,
                        Connect4Message::PlayerMove {
                            user_chain: chain_id,
                            column,
                        },
                    );
                    log::info!("Player {:?} making move in column {}", chain_id, column);
                } else {
                    log::error!("Not in a game. Cannot make move.");
                }
            }

            Connect4Operation::Surrender {} => {
                self.assert_user_chain(chain_type);
                let chain_id = self.runtime.chain_id();

                if let Some(game_chain) = self.state.user_game_chain.get().as_ref() {
                    self.message_manager(
                        *game_chain,
                        Connect4Message::PlayerSurrender { user_chain: chain_id },
                    );
                    log::info!("Player {:?} surrendering", chain_id);
                }
            }

            Connect4Operation::ExitGame {} => {
                self.assert_user_chain(chain_type);
                self.state.user_game_chain.set(None);
                self.state.user_color.set(None);
                self.state.channel_game_state.set(None);

                if let Some(ref mut profile) = *self.state.user_profile.get_mut() {
                    profile.set_status(UserStatus::Idle);
                }

                log::info!("Player exited game, returned to idle");
            }

            Connect4Operation::GetBalance {} => {
                self.assert_user_chain(chain_type);
                let balance = self.bankroll_get_balance();
                self.state.user_balance.set(balance);
                log::info!("GetBalance: {}", balance);
            }

            Connect4Operation::InitialSetup { lobby_chain } => {
                // First-time setup: set chain_type to User (3) if not already set
                if chain_type == 0 {
                    self.state.chain_type.set(3);
                    log::info!("InitialSetup: Set chain_type to User (3)");
                }

                // Store the lobby chain provided by the frontend config
                self.state.lobby_chain.set(Some(lobby_chain));
                self.state.cached_lobby_chain.set(Some(lobby_chain));

                log::info!("InitialSetup: Configured lobby chain {:?}", lobby_chain);
            }

            // ============================================
            // MASTER CHAIN OPERATIONS
            // ============================================
            Connect4Operation::AddLobbyChain { chain_id } => {
                self.assert_master_chain(chain_type);
                log::info!("Adding lobby chain: {:?}", chain_id);

                let info = abi::management::LobbyChainInfo::new(chain_id, self.runtime.system_time());
                self.state.lobby_chains.insert(&chain_id, info).expect("Failed to insert lobby chain");
            }

            Connect4Operation::AddGameChain { chain_id } => {
                self.assert_master_chain(chain_type);
                log::info!("Adding game chain: {:?}", chain_id);

                let lobby_keys = self.state.lobby_chains.indices().await.expect("Failed to get lobby chains");
                for lobby_chain in lobby_keys {
                    self.message_manager(
                        lobby_chain,
                        Connect4Message::RegisterGameChain { chain_id },
                    );
                }
            }

            Connect4Operation::MintToken { chain_id, amount } => {
                self.assert_master_chain(chain_type);
                log::info!("MintToken: {:?} amount: {}", chain_id, amount);
                self.bankroll_mint_token(chain_id, amount);
            }
        }
    }

    async fn execute_message(&mut self, message: Self::Message) {
        let chain_type = *self.state.chain_type.get();
        let origin = self.runtime.message_origin_chain_id().expect("No origin chain");

        match message {
            // ============================================
            // SUBSCRIPTION CONTROL
            // ============================================
            Connect4Message::Subscribe => {
                log::info!("Chain {:?} subscribing to events", origin);
                let app_id = self.runtime.application_id().forget_abi();
                self.runtime.subscribe_to_events(origin, app_id, CONNECT4_STREAM_NAME.into());
            }

            Connect4Message::Unsubscribe => {
                log::info!("Chain {:?} unsubscribing from events", origin);
                let app_id = self.runtime.application_id().forget_abi();
                self.runtime.unsubscribe_from_events(origin, app_id, CONNECT4_STREAM_NAME.into());
            }

            // ============================================
            // USER CHAIN MESSAGES
            // ============================================
            Connect4Message::MatchFound {
                game_chain,
                game_id,
                opponent_name,
                opponent_elo,
                your_color,
            } => {
                log::info!(
                    "Match found! Game: {}, Opponent: {} (ELO: {}), Your color: {:?}",
                    game_id, opponent_name, opponent_elo, your_color
                );

                self.state.user_game_chain.set(Some(game_chain));
                self.state.user_color.set(Some(your_color));

                if let Some(ref mut profile) = *self.state.user_profile.get_mut() {
                    profile.set_status(UserStatus::InGame { game_chain });
                }

                // Subscribe to game chain events
                let app_id = self.runtime.application_id().forget_abi();
                self.runtime.subscribe_to_events(game_chain, app_id, CONNECT4_STREAM_NAME.into());

                // Automatically join the game
                let chain_id = self.runtime.chain_id();
                self.message_manager(game_chain, Connect4Message::JoinGame { user_chain: chain_id });
            }

            Connect4Message::MoveMade {
                column,
                row,
                player,
                your_turn,
                board,
            } => {
                log::info!(
                    "Move made: {:?} dropped disc at column {}, row {}. Your turn: {}",
                    player, column, row, your_turn
                );

                // Update or initialize local game state cache
                let my_color = self.state.user_color.get().unwrap_or(Player::Red);
                let current_turn = if your_turn { my_color } else { my_color.opponent() };

                if let Some(ref mut game) = *self.state.channel_game_state.get_mut() {
                    game.board = board;
                    game.current_turn = current_turn;
                } else {
                    // First MoveMade message - initialize game state
                    let mut game = Connect4GameState::new(0);
                    game.board = board;
                    game.current_turn = current_turn;
                    game.status = GameStatus::InProgress;
                    self.state.channel_game_state.set(Some(game));
                }
            }

            Connect4Message::GameResult {
                winner,
                your_elo_change,
                new_elo,
            } => {
                let won = winner == Some(self.runtime.chain_id());

                log::info!(
                    "Game over! Winner: {:?}, ELO change: {}, New ELO: {}",
                    winner, your_elo_change, new_elo
                );

                if let Some(ref mut profile) = *self.state.user_profile.get_mut() {
                    profile.elo = new_elo;
                    profile.set_status(UserStatus::Idle);
                    profile.stats.record_game(won, 1);
                }

                // Clean up game state
                self.state.user_game_chain.set(None);
                self.state.user_color.set(None);
                self.state.channel_game_state.set(None);
            }

            Connect4Message::ProfileUpdated { profile } => {
                log::info!("Profile updated: {}", profile.name);
            }

            Connect4Message::LobbyInfo { lobby_chain } => {
                log::info!("Received lobby chain info: {:?}", lobby_chain);
                self.state.lobby_chain.set(Some(lobby_chain));

                let app_id = self.runtime.application_id().forget_abi();
                self.runtime.subscribe_to_events(lobby_chain, app_id, CONNECT4_STREAM_NAME.into());
            }

            // ============================================
            // LOBBY CHAIN MESSAGES
            // ============================================
            Connect4Message::JoinMatchmaking {
                user_chain,
                player_name,
                elo,
            } => {
                log::info!("Player {} ({:?}) joining matchmaking queue", player_name, user_chain);

                // CRITICAL FIX: Cross-chain messages don't have authenticated signer
                // Use application ID as owner identifier (ApplicationId can convert to AccountOwner)
                let app_id = self.runtime.application_id().forget_abi();
                let owner: linera_sdk::linera_base_types::AccountOwner = app_id.into();

                let player = QueuedPlayer::new(
                    user_chain,
                    owner,
                    player_name,
                    elo,
                    self.runtime.system_time(),
                );

                self.state.matchmaking_queue.push_back(player);
                let count = self.state.queue_count.get_mut();
                *count += 1;

                log::info!("🎯 MATCHMAKING QUEUE UPDATED: {} players now in queue", *count);

                self.runtime.emit(
                    CONNECT4_STREAM_NAME.into(),
                    &Connect4Event::QueueUpdate { players_in_queue: *count },
                );

                self.try_match_players().await;
            }

            Connect4Message::LeaveMatchmaking { user_chain } => {
                log::info!("Player {:?} leaving matchmaking queue", user_chain);

                let queue_count = *self.state.queue_count.get();
                let all_players: Vec<QueuedPlayer> = {
                    let mut players = Vec::with_capacity(queue_count as usize);
                    for _ in 0..queue_count {
                        if let Ok(Some(player)) = self.state.matchmaking_queue.front().await {
                            self.state.matchmaking_queue.delete_front();
                            players.push(player);
                        }
                    }
                    players
                };

                let mut removed = false;
                for player in all_players {
                    if player.chain_id != user_chain {
                        self.state.matchmaking_queue.push_back(player);
                    } else {
                        removed = true;
                    }
                }

                if removed {
                    let count = self.state.queue_count.get_mut();
                    *count = count.saturating_sub(1);

                    self.runtime.emit(
                        CONNECT4_STREAM_NAME.into(),
                        &Connect4Event::QueueUpdate { players_in_queue: *count },
                    );
                }
            }

            Connect4Message::GameEnded {
                game_id,
                winner,
                red_player: _,
                yellow_player: _,
            } => {
                log::info!("Game {} ended on game chain, winner: {:?}", game_id, winner);

                // Return game chain to pool
                // The game chain will be returned to the available pool by the game chain itself
            }

            Connect4Message::RegisterGameChain { chain_id } => {
                log::info!("Registering game chain: {:?}", chain_id);
                self.state.available_game_chains.push_back(chain_id);
            }

            // ============================================
            // GAME CHAIN MESSAGES
            // ============================================
            Connect4Message::AssignMatch {
                game_id,
                player1,
                player2,
            } => {
                log::info!("Assigned match {} with {} vs {}", game_id, player1.name, player2.name);

                let mut game = Connect4GameState::new(game_id);

                // Player1 is Red, Player2 is Yellow
                game.red_player = Some(Connect4Player::new(
                    player1.chain_id,
                    player1.name.clone(),
                    player1.elo,
                ));
                game.yellow_player = Some(Connect4Player::new(
                    player2.chain_id,
                    player2.name.clone(),
                    player2.elo,
                ));

                self.state.current_game.set(Some(game));
                self.state.game_chain_available.set(false);

                log::info!("Game {} created, waiting for players to join", game_id);
            }

            Connect4Message::JoinGame { user_chain } => {
                log::info!("Player {:?} joining game", user_chain);

                let timestamp = self.runtime.system_time();
                let mut should_start = false;
                let mut game_data: Option<(u64, ChainId, String, ChainId, String)> = None;

                if let Some(ref mut game) = *self.state.current_game.get_mut() {
                    // Mark player as joined
                    if let Some((player, _color)) = game.get_player_mut_by_chain(&user_chain) {
                        player.joined = true;
                        log::info!("Player {:?} marked as joined", user_chain);
                    }

                    // Check if both players joined
                    if game.both_players_joined() {
                        game.start(timestamp);
                        should_start = true;

                        if let (Some(ref red), Some(ref yellow)) = (&game.red_player, &game.yellow_player) {
                            game_data = Some((
                                game.game_id,
                                red.chain_id,
                                red.name.clone(),
                                yellow.chain_id,
                                yellow.name.clone(),
                            ));
                        }
                    }
                }

                if should_start {
                    if let Some((game_id, red_chain, red_name, yellow_chain, yellow_name)) = game_data {
                        log::info!("Both players joined - starting game {}", game_id);

                        // Emit game started event
                        self.runtime.emit(
                            CONNECT4_STREAM_NAME.into(),
                            &Connect4Event::GameStarted {
                                game_id,
                                red_player: red_chain,
                                red_name: red_name.clone(),
                                yellow_player: yellow_chain,
                                yellow_name: yellow_name.clone(),
                            },
                        );

                        // Get board state for messages
                        let board = if let Some(ref game) = *self.state.current_game.get() {
                            game.board
                        } else {
                            [[None; 7]; 6]
                        };

                        // Notify Red player (their turn first)
                        self.message_manager(
                            red_chain,
                            Connect4Message::MoveMade {
                                column: 0,
                                row: 0,
                                player: Player::Red,
                                your_turn: true,
                                board,
                            },
                        );

                        // Notify Yellow player (not their turn)
                        self.message_manager(
                            yellow_chain,
                            Connect4Message::MoveMade {
                                column: 0,
                                row: 0,
                                player: Player::Red,
                                your_turn: false,
                                board,
                            },
                        );
                    }
                }
            }

            Connect4Message::PlayerMove { user_chain, column } => {
                log::info!("Player {:?} attempting move in column {}", user_chain, column);

                let timestamp = self.runtime.system_time();
                let result = self.process_move(user_chain, column, timestamp);

                match result {
                    MoveResult::Success { row, board, next_turn } => {
                        log::info!("Move successful: disc landed at row {}", row);

                        // Collect data needed for messages first
                        let send_data = {
                            if let Some(ref game) = *self.state.current_game.get() {
                                let player_color = game.get_player_by_chain(&user_chain)
                                    .map(|(_, c)| c)
                                    .unwrap_or(Player::Red);
                                let red_chain = game.red_player.as_ref().map(|p| p.chain_id);
                                let yellow_chain = game.yellow_player.as_ref().map(|p| p.chain_id);
                                let game_id = game.game_id;
                                Some((player_color, red_chain, yellow_chain, game_id))
                            } else {
                                None
                            }
                        };

                        // Now send messages with collected data
                        if let Some((player_color, red_chain, yellow_chain, game_id)) = send_data {
                            if let Some(rc) = red_chain {
                                self.message_manager(
                                    rc,
                                    Connect4Message::MoveMade {
                                        column,
                                        row: row as u8,
                                        player: player_color,
                                        your_turn: next_turn == Player::Red,
                                        board,
                                    },
                                );
                            }

                            if let Some(yc) = yellow_chain {
                                self.message_manager(
                                    yc,
                                    Connect4Message::MoveMade {
                                        column,
                                        row: row as u8,
                                        player: player_color,
                                        your_turn: next_turn == Player::Yellow,
                                        board,
                                    },
                                );
                            }

                            // Emit move event
                            self.runtime.emit(
                                CONNECT4_STREAM_NAME.into(),
                                &Connect4Event::MoveUpdate {
                                    game_id,
                                    move_made: Move {
                                        player: player_color,
                                        column,
                                        row: row as u8,
                                        timestamp: timestamp.micros(),
                                    },
                                    is_winning_move: false,
                                },
                            );
                        }
                    }

                    MoveResult::Win { row: _, board: _, winner } => {
                        log::info!("WINNER: {:?} wins the game!", winner);
                        self.handle_game_end(winner, GameEndReason::FourInARow, timestamp).await;
                    }

                    MoveResult::Draw { board: _ } => {
                        log::info!("DRAW: Board is full, no winner");
                        self.handle_game_end(Player::Red, GameEndReason::Draw, timestamp).await;
                    }

                    MoveResult::InvalidMove { reason } => {
                        log::error!("Invalid move: {}", reason);
                    }

                    MoveResult::GameNotActive => {
                        log::error!("Cannot make move: game is not active");
                    }
                }
            }

            Connect4Message::PlayerSurrender { user_chain } => {
                log::info!("Player {:?} surrendered", user_chain);

                let timestamp = self.runtime.system_time();

                // Find the surrendering player's color and declare opponent winner
                if let Some(ref game) = *self.state.current_game.get() {
                    if let Some((_, color)) = game.get_player_by_chain(&user_chain) {
                        let winner = color.opponent();
                        self.handle_game_end(winner, GameEndReason::Surrender, timestamp).await;
                    }
                }
            }

            // ============================================
            // MASTER CHAIN MESSAGES
            // ============================================
            Connect4Message::RequestLobbyInfo { user_chain } => {
                log::info!("Lobby info requested by {:?}", user_chain);

                let lobby_keys = self.state.lobby_chains.indices().await.expect("Failed to get lobby chains");
                if let Some(lobby_chain) = lobby_keys.into_iter().next() {
                    self.message_manager(
                        user_chain,
                        Connect4Message::LobbyInfo { lobby_chain },
                    );
                }
            }

            Connect4Message::UpdateLeaderboard {
                winner,
                winner_name,
                winner_new_elo,
                loser,
                loser_name,
                loser_new_elo,
            } => {
                log::info!(
                    "Updating leaderboard - Winner: {} (ELO: {}), Loser: {} (ELO: {})",
                    winner_name, winner_new_elo, loser_name, loser_new_elo
                );

                // Update winner entry
                let mut winner_entry = self.state.leaderboard.get(&winner).await
                    .expect("Failed to load winner leaderboard entry")
                    .unwrap_or_else(|| SimpleLeaderboardEntry {
                        player_id: Some(winner),
                        player_name: winner_name.clone(),
                        rank: 0,
                        elo: STARTING_ELO,
                        games_won: 0,
                        games_played: 0,
                        win_rate: 0,
                    });

                winner_entry.games_won += 1;
                winner_entry.games_played += 1;
                winner_entry.elo = winner_new_elo;
                winner_entry.win_rate = if winner_entry.games_played > 0 {
                    (winner_entry.games_won * 10000) / winner_entry.games_played
                } else {
                    0
                };
                winner_entry.player_name = winner_name;

                // Update loser entry
                let mut loser_entry = self.state.leaderboard.get(&loser).await
                    .expect("Failed to load loser leaderboard entry")
                    .unwrap_or_else(|| SimpleLeaderboardEntry {
                        player_id: Some(loser),
                        player_name: loser_name.clone(),
                        rank: 0,
                        elo: STARTING_ELO,
                        games_won: 0,
                        games_played: 0,
                        win_rate: 0,
                    });

                loser_entry.games_played += 1;
                loser_entry.elo = loser_new_elo;
                loser_entry.win_rate = if loser_entry.games_played > 0 {
                    (loser_entry.games_won * 10000) / loser_entry.games_played
                } else {
                    0
                };
                loser_entry.player_name = loser_name;

                // Store updated entries
                self.state.leaderboard.insert(&winner, winner_entry.clone())
                    .expect("Failed to update winner leaderboard entry");
                self.state.leaderboard.insert(&loser, loser_entry.clone())
                    .expect("Failed to update loser leaderboard entry");

                // Emit leaderboard update event
                self.runtime.emit(
                    CONNECT4_STREAM_NAME.into(),
                    &Connect4Event::LeaderboardUpdate {
                        entries: vec![winner_entry, loser_entry],
                    },
                );
            }
        }
    }

    async fn process_streams(&mut self, updates: Vec<linera_sdk::linera_base_types::StreamUpdate>) {
        for update in updates {
            assert_eq!(
                update.stream_id.stream_name,
                CONNECT4_STREAM_NAME.into(),
                "Unexpected stream name"
            );

            for index in update.new_indices() {
                let event: Connect4Event = self
                    .runtime
                    .read_event(update.chain_id, CONNECT4_STREAM_NAME.into(), index);

                log::debug!("Received event from chain {:?}: {:?}", update.chain_id, event);

                match event {
                    Connect4Event::GameState {
                        game_id,
                        board,
                        status,
                        current_turn,
                        winner,
                        move_history,
                    } => {
                        log::info!("Game state update: game_id={}, status={:?}", game_id, status);

                        // Update local cache
                        if let Some(ref mut game) = *self.state.channel_game_state.get_mut() {
                            game.board = board;
                            game.status = status;
                            game.current_turn = current_turn;
                            game.winner = winner;
                            game.move_history = move_history;
                        }
                    }

                    Connect4Event::QueueUpdate { players_in_queue } => {
                        self.state.queue_count.set(players_in_queue);
                        log::info!("Queue updated: {} players waiting", players_in_queue);
                    }

                    Connect4Event::LeaderboardUpdate { entries } => {
                        log::info!("Leaderboard updated with {} entries", entries.len());
                    }

                    Connect4Event::ProfileUpdate { profile } => {
                        log::info!("Profile update received: {}", profile.name);
                    }

                    Connect4Event::MoveUpdate {
                        game_id,
                        move_made,
                        is_winning_move,
                    } => {
                        log::info!(
                            "Move in game {}: {:?} column {}, row {} (winning: {})",
                            game_id, move_made.player, move_made.column, move_made.row, is_winning_move
                        );
                    }

                    Connect4Event::GameStarted {
                        game_id,
                        red_player: _,
                        red_name,
                        yellow_player: _,
                        yellow_name,
                    } => {
                        log::info!(
                            "Game {} started: {} (Red) vs {} (Yellow)",
                            game_id, red_name, yellow_name
                        );
                    }

                    Connect4Event::GameEnded {
                        game_id,
                        winner,
                        end_reason,
                    } => {
                        log::info!(
                            "Game {} ended: winner={:?}, reason={:?}",
                            game_id, winner, end_reason
                        );
                    }
                }
            }
        }
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}

/// Result of processing a move
#[allow(dead_code)]
enum MoveResult {
    /// Move was successful
    Success {
        row: usize,
        board: [[Option<Player>; 7]; 6],
        next_turn: Player,
    },
    /// Move resulted in a win
    Win {
        row: usize,
        board: [[Option<Player>; 7]; 6],
        winner: Player,
    },
    /// Move resulted in a draw
    Draw {
        board: [[Option<Player>; 7]; 6],
    },
    /// Move was invalid
    InvalidMove {
        reason: String,
    },
    /// Game is not active
    GameNotActive,
}

impl Connect4Contract {
    /// Assert this is a user chain (type 3)
    fn assert_user_chain(&self, chain_type: u64) {
        assert!(
            chain_type == 3 || chain_type == 0,
            "Operation requires User chain (type 3), got type {}",
            chain_type
        );
    }

    /// Assert this is a master chain (type 0)
    fn assert_master_chain(&self, chain_type: u64) {
        assert_eq!(
            chain_type, 0,
            "Operation requires Master chain (type 0), got type {}",
            chain_type
        );
    }

    /// Assert this is a game chain (type 0 allowed for single-chain Docker deployment)
    fn assert_game_chain(&self, chain_type: u64) {
        assert!(
            chain_type == 2 || chain_type == 0,
            "Operation requires Game chain (type 2) or Master chain (type 0), got type {}",
            chain_type
        );
    }

    /// Send a message to another chain with tracking
    fn message_manager(&mut self, destination: ChainId, message: Connect4Message) {
        self.runtime
            .prepare_message(message)
            .with_tracking()
            .send_to(destination);
    }

    /// Process a move and return the result
    fn process_move(
        &mut self,
        player_chain: ChainId,
        column: u8,
        timestamp: linera_sdk::linera_base_types::Timestamp,
    ) -> MoveResult {
        let game = self.state.current_game.get_mut();

        if game.is_none() {
            return MoveResult::GameNotActive;
        }

        let game = game.as_mut().unwrap();

        // Check game is in progress
        if game.status != GameStatus::InProgress {
            return MoveResult::InvalidMove {
                reason: format!("Game is not in progress (status: {:?})", game.status),
            };
        }

        // Get player color
        let player_color = match game.get_player_by_chain(&player_chain) {
            Some((_, color)) => color,
            None => {
                return MoveResult::InvalidMove {
                    reason: "Player not found in this game".to_string(),
                };
            }
        };

        // Check it's the player's turn
        if !game.is_player_turn(player_color) {
            return MoveResult::InvalidMove {
                reason: format!("Not your turn. Current turn: {:?}", game.current_turn),
            };
        }

        // Attempt to drop the disc
        let row = match drop_disc(&mut game.board, column as usize, player_color) {
            Some(r) => r,
            None => {
                return MoveResult::InvalidMove {
                    reason: format!("Column {} is full or invalid", column),
                };
            }
        };

        // Record the move
        game.move_history.push(Move {
            player: player_color,
            column,
            row: row as u8,
            timestamp: timestamp.micros(),
        });

        // Check for win
        if check_winner(&game.board, row, column as usize) {
            game.status = GameStatus::Finished;
            game.winner = Some(player_color);
            game.ended_at = Some(timestamp);

            return MoveResult::Win {
                row,
                board: game.board,
                winner: player_color,
            };
        }

        // Check for draw
        if is_board_full(&game.board) {
            game.status = GameStatus::Draw;
            game.ended_at = Some(timestamp);

            return MoveResult::Draw {
                board: game.board,
            };
        }

        // Switch turn
        game.switch_turn();

        MoveResult::Success {
            row,
            board: game.board,
            next_turn: game.current_turn,
        }
    }

    /// Handle game end - calculate ELO, notify players, update leaderboard
    async fn handle_game_end(
        &mut self,
        winner_color: Player,
        reason: GameEndReason,
        timestamp: linera_sdk::linera_base_types::Timestamp,
    ) {
        let game_data = {
            let game = self.state.current_game.get_mut();
            if game.is_none() {
                return;
            }

            let game = game.as_mut().unwrap();

            // Update game status
            if reason != GameEndReason::Draw {
                game.status = GameStatus::Finished;
                game.winner = Some(winner_color);
            } else {
                game.status = GameStatus::Draw;
                game.winner = None;
            }
            game.ended_at = Some(timestamp);

            // Extract player data
            let red = game.red_player.clone();
            let yellow = game.yellow_player.clone();
            let game_id = game.game_id;
            let board = game.board;

            (game_id, red, yellow, board)
        };

        let (game_id, red_opt, yellow_opt, _board) = game_data;

        if let (Some(red), Some(yellow)) = (red_opt, yellow_opt) {
            // Determine winner and loser chains
            let (winner_chain, winner_name, winner_elo, loser_chain, loser_name, loser_elo) = match winner_color {
                Player::Red => (red.chain_id, red.name.clone(), red.elo, yellow.chain_id, yellow.name.clone(), yellow.elo),
                Player::Yellow => (yellow.chain_id, yellow.name.clone(), yellow.elo, red.chain_id, red.name.clone(), red.elo),
            };

            // Calculate ELO change
            let elo_change = if reason == GameEndReason::Draw {
                0
            } else {
                calculate_elo_change(winner_elo, loser_elo, true)
            };

            let winner_new_elo = (winner_elo as i32 + elo_change) as u32;
            let loser_new_elo = (loser_elo as i32 - elo_change.abs()).max(100) as u32;

            // Notify winner
            self.message_manager(
                winner_chain,
                Connect4Message::GameResult {
                    winner: if reason == GameEndReason::Draw { None } else { Some(winner_chain) },
                    your_elo_change: elo_change,
                    new_elo: winner_new_elo,
                },
            );

            // Notify loser
            self.message_manager(
                loser_chain,
                Connect4Message::GameResult {
                    winner: if reason == GameEndReason::Draw { None } else { Some(winner_chain) },
                    your_elo_change: -elo_change,
                    new_elo: loser_new_elo,
                },
            );

            // Emit game ended event
            self.runtime.emit(
                CONNECT4_STREAM_NAME.into(),
                &Connect4Event::GameEnded {
                    game_id,
                    winner: if reason == GameEndReason::Draw { None } else { Some(winner_chain) },
                    end_reason: reason,
                },
            );

            // Notify lobby that game ended
            let lobby_chain = self.get_lobby_chain();
            self.message_manager(
                lobby_chain,
                Connect4Message::GameEnded {
                    game_id,
                    winner: if reason == GameEndReason::Draw { None } else { Some(winner_chain) },
                    red_player: red.chain_id,
                    yellow_player: yellow.chain_id,
                },
            );

            // Update leaderboard on master chain (only if not a draw)
            if reason != GameEndReason::Draw {
                let master_chain = self.get_master_chain();
                self.message_manager(
                    master_chain,
                    Connect4Message::UpdateLeaderboard {
                        winner: winner_chain,
                        winner_name,
                        winner_new_elo,
                        loser: loser_chain,
                        loser_name,
                        loser_new_elo,
                    },
                );
            }

            log::info!(
                "Game {} ended: {:?} wins (reason: {:?}), ELO change: {}",
                game_id, winner_color, reason, elo_change
            );
        }

        // Mark game chain as available
        self.state.game_chain_available.set(true);
        let games_hosted = self.state.games_hosted.get_mut();
        *games_hosted += 1;
        self.state.current_game.set(None);
    }

    /// Try to match players in the queue
    async fn try_match_players(&mut self) {
        let queue_count = *self.state.queue_count.get();
        if queue_count < 2 {
            return;
        }

        // Get two players from queue
        let player1 = match self.state.matchmaking_queue.front().await {
            Ok(Some(p)) => p,
            _ => return,
        };
        self.state.matchmaking_queue.delete_front();

        let player2 = match self.state.matchmaking_queue.front().await {
            Ok(Some(p)) => p,
            _ => {
                self.state.matchmaking_queue.push_back(player1);
                return;
            }
        };
        self.state.matchmaking_queue.delete_front();

        // Update queue count
        let count = self.state.queue_count.get_mut();
        *count = count.saturating_sub(2);

        // Get available game chain
        let game_chain = match self.state.available_game_chains.front().await {
            Ok(Some(gc)) => {
                self.state.available_game_chains.delete_front();
                gc
            }
            _ => {
                // Use current chain for single-chain deployment
                log::info!("No registered game chains, using current chain for game");
                self.runtime.chain_id()
            }
        };

        // Track active game chain
        let game_chain_info = abi::management::GameChainInfo::new(game_chain, self.runtime.system_time());
        self.state.active_game_chains.insert(&game_chain, game_chain_info).expect("Failed to insert game chain");

        // Create game ID
        let game_id = self.runtime.system_time().micros();

        log::info!(
            "Matched {} (ELO: {}) vs {} (ELO: {}) on game chain {:?}",
            player1.name, player1.elo, player2.name, player2.elo, game_chain
        );

        // Notify players of match - player1 is Red, player2 is Yellow
        self.message_manager(
            player1.chain_id,
            Connect4Message::MatchFound {
                game_chain,
                game_id,
                opponent_name: player2.name.clone(),
                opponent_elo: player2.elo,
                your_color: Player::Red,
            },
        );

        self.message_manager(
            player2.chain_id,
            Connect4Message::MatchFound {
                game_chain,
                game_id,
                opponent_name: player1.name.clone(),
                opponent_elo: player1.elo,
                your_color: Player::Yellow,
            },
        );

        // Assign match to game chain
        self.message_manager(
            game_chain,
            Connect4Message::AssignMatch {
                game_id,
                player1,
                player2,
            },
        );

        self.runtime.emit(
            CONNECT4_STREAM_NAME.into(),
            &Connect4Event::QueueUpdate { players_in_queue: *self.state.queue_count.get() },
        );
    }

    // ============================================
    // BANKROLL INTEGRATION HELPERS
    // ============================================

    /// Get balance from bankroll application
    fn bankroll_get_balance(&mut self) -> Amount {
        let owner = self.runtime.application_id().into();
        let bankroll_app_id = self.get_bankroll();
        let response = self.runtime.call_application(true, bankroll_app_id, &BankrollOperation::Balance { owner });
        match response {
            BankrollResponse::Balance(balance) => balance,
            response => {
                log::error!("Unexpected response from Bankroll application: {:?}", response);
                Amount::ZERO
            }
        }
    }

    /// Mint tokens via bankroll application (master chain only)
    fn bankroll_mint_token(&mut self, chain_id: ChainId, amount: Amount) {
        let bankroll_app_id = self.get_bankroll();
        let _ = self.runtime.call_application(true, bankroll_app_id, &BankrollOperation::MintToken { chain_id, amount });
        log::info!("Minted {} tokens for chain {:?}", amount, chain_id);
    }

    // ============================================
    // PARAMETER HELPERS - Cached to avoid runtime.application_parameters() issues
    // ============================================

    /// Get master chain ID with caching
    fn get_master_chain(&mut self) -> ChainId {
        if let Some(chain_id) = *self.state.cached_master_chain.get() {
            chain_id
        } else {
            let params = self.runtime.application_parameters();
            self.state.cached_master_chain.set(Some(params.master_chain));
            params.master_chain
        }
    }

    /// Get bankroll application ID with caching
    fn get_bankroll(&mut self) -> ApplicationId<BankrollAbi> {
        if let Some(app_id) = *self.state.cached_bankroll.get() {
            app_id
        } else {
            let params = self.runtime.application_parameters();
            self.state.cached_bankroll.set(Some(params.bankroll));
            params.bankroll
        }
    }

    /// Get lobby chain ID with caching
    fn get_lobby_chain(&mut self) -> ChainId {
        if let Some(chain_id) = *self.state.cached_lobby_chain.get() {
            chain_id
        } else {
            let params = self.runtime.application_parameters();
            self.state.cached_lobby_chain.set(Some(params.lobby_chain));
            params.lobby_chain
        }
    }
}
