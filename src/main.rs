mod classes;
mod enums;

use classes::game::Game;
use classes::screen_manager::{ScreenManager, ScreenAction};
use eframe::egui;

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
        Box::new(|_cc| Ok(Box::new(TetrisApp::new()))),
    )
}

struct TetrisApp {
    game: Game,
    screen_manager: ScreenManager,
}

impl TetrisApp {
    fn new() -> Self {
        Self {
            game: Game::new(),
            screen_manager: ScreenManager::new(),
        }
    }
}

impl eframe::App for TetrisApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update game logic and handle input
        self.game.update(ctx);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rusttris");
            ui.add_space(10.0);
            
            ui.vertical_centered(|ui| {
                // Let screen manager handle all screen rendering
                if let Some(action) = self.screen_manager.draw(&mut self.game, ui) {
                    match action {
                        ScreenAction::StartGame => self.game.start_game(),
                        ScreenAction::RestartGame => self.game.reset_game(),
                    }
                }
            });

            ui.add_space(10.0);
            ui.label("Use arrow keys to play!");
        });
    }
}
