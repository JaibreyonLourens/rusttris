use eframe::egui;
use super::screens;
use crate::enums::states::GameState;
use super::game::Game;
use super::database::database::DbManager;

pub struct ScreenManager {
    player_name_input: String,
}

impl ScreenManager {
    pub fn new() -> Self {
        ScreenManager {
            player_name_input: String::new(),
        }
    }

    pub fn draw(&mut self, game: &mut Game, ui: &mut egui::Ui, player_name: &str, db_manager: &DbManager) -> Option<ScreenAction> {
        match game.get_state() {
            GameState::PlayerCreation => {
                if screens::player_creation::draw(ui, &mut self.player_name_input) {
                    let name = self.player_name_input.trim().to_string();
                    self.player_name_input.clear();
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
                if let Some(action) = screens::menu::draw(ui, player_name) {
                    return Some(match action {
                        screens::menu::MenuAction::StartGame => ScreenAction::StartGame,
                        screens::menu::MenuAction::NewPlayer => ScreenAction::ShowPlayerCreation,
                        screens::menu::MenuAction::SelectPlayer => ScreenAction::ShowPlayerSelection,
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
                screens::paused::draw(ui);
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
    CreatePlayer(String),
    ShowPlayerCreation,
    SelectPlayer(i64),
    ShowPlayerSelection,
    BackToMenu,
}
