use eframe::egui;
use super::screens;
use crate::enums::states::GameState;
use super::game::Game;
use super::database::database::DbManager;

pub struct ScreenManager {
    player_name_input: String,
    leaderboard_state: screens::leaderboard::LeaderboardState,
    pub player_creation_error: Option<String>,
    options_screen: screens::options::OptionsScreen,
}

impl ScreenManager {
    pub fn new() -> Self {
        ScreenManager {
            player_name_input: String::new(),
            leaderboard_state: screens::leaderboard::LeaderboardState::new(),
            player_creation_error: None,
            options_screen: screens::options::OptionsScreen::default(),
        }
    }

    pub fn draw(&mut self, game: &mut Game, ui: &mut egui::Ui, player_name: &str, db_manager: &DbManager, options: &mut crate::classes::game_options::GameOptions) -> Option<ScreenAction> {
        match game.get_state() {
            GameState::PlayerCreation => {
                if screens::player_creation::draw(ui, &mut self.player_name_input, self.player_creation_error.as_deref()) {
                    let name = self.player_name_input.trim().to_string();
                    self.player_name_input.clear();
                    self.player_creation_error = None;
                    return Some(ScreenAction::CreatePlayer(name));
                }
                None
            },
            GameState::PlayerSelection => {
                // Fetch all players from database
                let players = db_manager.get_all_players()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|(id, name)| screens::player_selection::PlayerInfo { id, name })
                    .collect::<Vec<_>>();
                
                if let Some(action) = screens::player_selection::draw_with_players(ui, &players) {
                    return Some(match action {
                        screens::player_selection::PlayerSelectionAction::SelectPlayer(player_id) => {
                            ScreenAction::SelectPlayer(player_id)
                        },
                        screens::player_selection::PlayerSelectionAction::Back => {
                            ScreenAction::BackToMenu
                        },
                    });
                }
                None
            },
            GameState::Menu => {
                let has_active_game = game.is_game_active();
                if let Some(action) = screens::menu::draw(ui, player_name, has_active_game) {
                    return Some(match action {
                        screens::menu::MenuAction::StartGame => ScreenAction::StartGame,
                        screens::menu::MenuAction::ResumeGame => ScreenAction::ResumeGame,
                        screens::menu::MenuAction::NewPlayer => ScreenAction::ShowPlayerCreation,
                        screens::menu::MenuAction::SelectPlayer => ScreenAction::ShowPlayerSelection,
                        screens::menu::MenuAction::ShowLeaderboard => ScreenAction::ShowLeaderboard,
                        screens::menu::MenuAction::ShowOptions => ScreenAction::ShowOptions,
                    });
                }
                None
            },
            GameState::Leaderboard => {
                // Fetch leaderboard data for current category
                let leaderboard_data = db_manager
                    .get_leaderboard(10, self.leaderboard_state.current_category)
                    .unwrap_or_default();
                
                if let Some(action) = screens::leaderboard::draw(ui, &mut self.leaderboard_state, &leaderboard_data) {
                    return Some(match action {
                        screens::leaderboard::LeaderboardAction::Back => ScreenAction::BackToMenu,
                    });
                }
                None
            },
            GameState::Options => {
                if let Some(action) = self.options_screen.draw(ui, options) {
                    return Some(match action {
                        screens::options::OptionSelectionAction::Save => {
                            // Save options to file
                            if let Err(e) = options.save() {
                                println!("Error saving options: {}", e);
                            }
                            ScreenAction::BackToMenu
                        },
                        screens::options::OptionSelectionAction::Cancel => ScreenAction::BackToMenu,
                    });
                }
                None
            },
            GameState::Playing => {
                game.draw_game_board(ui);
                None
            },
            GameState::Paused => {
                game.draw_game_board(ui);
                if let Some(action) = screens::paused::draw(ui) {
                    return Some(match action {
                        screens::paused::PausedAction::Resume => ScreenAction::ResumeGame,
                        screens::paused::PausedAction::Restart => ScreenAction::RestartGame,
                        screens::paused::PausedAction::BackToMenu => ScreenAction::BackToMenu,
                    });
                } 
                None
            },
            GameState::GameOver => {
                game.draw_game_board(ui);
                if screens::game_over::draw(ui, game.get_score(), game.get_lines_cleared()) {
                    return Some(ScreenAction::RestartGame);
                }
                None
            },
        }
    }
}

pub enum ScreenAction {
    StartGame,
    RestartGame,
    ResumeGame,
    CreatePlayer(String),
    CreatePlayerError(String),
    ShowPlayerCreation,
    SelectPlayer(i64),
    ShowPlayerSelection,
    ShowLeaderboard,
    ShowOptions,
    BackToMenu,
}
