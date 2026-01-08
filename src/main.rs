mod classes;
mod enums;

use crate::classes::database::database::DbManager;

use classes::game::Game;
use classes::screen_manager::{ScreenManager, ScreenAction};
use classes::player::Player;
use eframe::egui;

use crate::classes::game_options::GameOptions;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 700.0])
            .with_title("Rusttris - Tetris in Rust"),
        ..Default::default()
    };

    eframe::run_native(
        "Rusttris",
        options,
        Box::new(|_cc| Ok(Box::new(RusttrisApp::new()))),
    )
}

struct RusttrisApp {
    game: Game,
    db_manager: DbManager,
    screen_manager: ScreenManager,
    player: Player,
    has_player: bool,
}

impl RusttrisApp {
    fn new() -> Self {
        let options = GameOptions::load();
        let db_manager = DbManager::new();
        db_manager.init_schemas();
        
        let mut game = Game::new(options);
        
        let player_result = db_manager.get_last_active_player();
        
        println!("Loading last active player...");
        let (player, has_player) = match player_result {
            Ok(Some(p)) => {
                println!("Loaded player: {} (ID: {:?})", p.name, p.id);
                game.set_state(crate::enums::states::GameState::Menu);
                (p, true)
            },
            Ok(None) => {
                println!("No players found in database");
                game.set_state(crate::enums::states::GameState::PlayerCreation);
                (Player::new("".to_string()), false)
            },
            Err(e) => {
                println!("Error loading player: {}", e);
                game.set_state(crate::enums::states::GameState::PlayerCreation);
                (Player::new("".to_string()), false)
            }
        };

        Self {
            game,
            screen_manager: ScreenManager::new(),
            db_manager,
            player,
            has_player,
        }
    }
}

impl eframe::App for RusttrisApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check if game just ended
        let previous_state = self.game.get_state();
        
        // Update game logic and handle input
        self.game.update(ctx);
        
        // Check if game transitioned to game over
        let current_state = self.game.get_state();
        if previous_state != crate::enums::states::GameState::GameOver 
            && current_state == crate::enums::states::GameState::GameOver {
            // Game just ended, save stats
            let game_stats = self.game.get_game_stats();
            
            if let Some(player_id) = self.player.id {
                println!("Saving game for player ID: {}", player_id);
                self.db_manager.save_game(player_id, &game_stats).unwrap();
            } else {
                println!("WARNING: Player has no ID, game not saved to database!");
            }
            
            self.player.finish_game(game_stats);
            
            // Print player stats
            println!("\n=== Player Statistics ===");
            println!("Total Games: {}", self.player.stats.total_games);
            println!("Highest Score: {}", self.player.stats.highest_score);
            println!("Highest Level: {}", self.player.stats.highest_level);
            println!("Average Score: {:.1}", self.player.stats.get_average_score());
            println!("Total Lines: {}", self.player.stats.total_lines);
        }
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rusttris");
            ui.add_space(10.0);
            
            ui.vertical_centered(|ui| {
                // Let screen manager handle all screen rendering
                let game_ptr = &mut self.game as *mut Game;
                let options_ptr = unsafe { &mut (*game_ptr).options as *mut GameOptions };
                if let Some(action) = self.screen_manager.draw(&mut self.game, ui, &self.player.name, &self.db_manager, unsafe { &mut *options_ptr }) {
                    match action {
                        ScreenAction::StartGame => self.game.start_game(),
                        ScreenAction::ResumeGame => self.game.resume_game(),
                        ScreenAction::RestartGame => self.game.reset_game(),
                        ScreenAction::CreatePlayer(name) => {
                            // Save to database here
                            match self.db_manager.create_player(&name) {
                                Ok(player_id) => {
                                    // Create player instance with DB id
                                    let mut player = Player::new(name.clone());
                                    player.id = Some(player_id);
                                    self.player = player;
                                    self.has_player = true;
                                    self.game.set_state(crate::enums::states::GameState::Menu);
                                    println!("Player created: {}", self.player.name);
                                },
                                Err(e) => {
                                    let error_msg = if e.to_string().contains("UNIQUE constraint") {
                                        format!("Player '{}' already exists. Please choose a different name.", name)
                                    } else {
                                        format!("Failed to create player: {}", e)
                                    };
                                    println!("{}", error_msg);
                                    // Send error back to screen manager
                                    self.screen_manager.player_creation_error = Some(error_msg);
                                }
                            }
                        },
                        ScreenAction::CreatePlayerError(_) => {
                            // Handled by screen_manager
                        },
                        ScreenAction::ShowPlayerCreation => {
                            self.game.set_state(crate::enums::states::GameState::PlayerCreation);
                        },
                        ScreenAction::ShowPlayerSelection => {
                            self.game.set_state(crate::enums::states::GameState::PlayerSelection);
                        },
                        ScreenAction::SelectPlayer(player_id) => {
                            println!("Selecting player with ID: {}", player_id);
                            if let Ok(Some(player)) = self.db_manager.get_player_with_stats(player_id) {
                                println!("Loaded player: {} (ID: {:?})", player.name, player.id);
                                self.player = player;
                                self.has_player = true;
                                self.game.set_state(crate::enums::states::GameState::Menu);
                            } else {
                                println!("Failed to load player with ID: {}", player_id);
                            }
                        },
                        ScreenAction::BackToMenu => {
                            self.game.set_state(crate::enums::states::GameState::Menu);
                        },
                        ScreenAction::ShowLeaderboard => {
                            self.game.set_state(crate::enums::states::GameState::Leaderboard);
                        },
                        ScreenAction::ShowOptions => {
                            self.game.set_state(crate::enums::states::GameState::Options);
                        },
                    }
                }
            });

            ui.add_space(10.0);
            ui.label("Use arrow keys to play!");
        });
    }
}
