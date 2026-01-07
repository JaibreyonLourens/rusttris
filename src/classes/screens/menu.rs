use eframe::egui;

pub enum MenuAction {
    StartGame,
    NewPlayer,
    SelectPlayer,
    ShowLeaderboard,
}

pub fn draw(ui: &mut egui::Ui, player_name: &str) -> Option<MenuAction> {
    let mut action = None;
    
    ui.vertical_centered(|ui| {
        ui.add_space(100.0);
        ui.heading("Welcome to Rusttris!");
        ui.add_space(20.0);
        
        ui.label(format!("Player: {}", player_name));
        ui.add_space(30.0);
        
        if ui.button("Start Game").clicked() {
            action = Some(MenuAction::StartGame);
        }
        
        ui.add_space(10.0);
        
        if ui.button("Leaderboard").clicked() {
            action = Some(MenuAction::ShowLeaderboard);
        }
        
        ui.add_space(10.0);
        
        if ui.button("Select Player").clicked() {
            action = Some(MenuAction::SelectPlayer);
        }
        
        ui.add_space(10.0);
        
        if ui.button("New Player").clicked() {
            action = Some(MenuAction::NewPlayer);
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
    
    action
}
