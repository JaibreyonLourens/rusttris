use eframe::egui;
use crate::enums::states::GameState;

pub fn draw(ui: &mut egui::Ui) -> Option<GameState> {
    let mut next_state = None;
    
    ui.vertical_centered(|ui| {
        ui.add_space(100.0);
        ui.heading("Welcome to Rusttris!");
        ui.add_space(50.0);
        
        if ui.button("Start Game").clicked() {
            next_state = Some(GameState::Playing);
        }
        
        ui.add_space(20.0);
        
        ui.label("Controls:");
        ui.label("← → : Move piece");
        ui.label("↓ : Soft drop");
        ui.label("↑ or X : Rotate clockwise");
        ui.label("Z : Rotate counterclockwise");
        ui.label("Space : Hard drop");
        ui.label("C : Hold piece");
        ui.label("P : Pause");
    });
    
    next_state
}
