use eframe::egui;
use super::screens;
use crate::enums::states::GameState;
use super::game::Game;

pub struct ScreenManager;

impl ScreenManager {
    pub fn new() -> Self {
        ScreenManager
    }

    pub fn draw(&self, game: &mut Game, ui: &mut egui::Ui) -> Option<ScreenAction> {
        match game.get_state() {
            GameState::Menu => {
                if screens::menu::draw(ui).is_some() {
                    return Some(ScreenAction::StartGame);
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
}
