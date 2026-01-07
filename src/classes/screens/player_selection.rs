use eframe::egui;

pub enum PlayerSelectionAction {
    SelectPlayer(i64),
    Back,
}

pub struct PlayerInfo {
    pub id: i64,
    pub name: String,
}

pub fn draw(ui: &mut egui::Ui) -> Option<PlayerSelectionAction> {
    // This will be called from screen_manager which will pass the players list
    // For now, we'll update this to accept players as parameter
    draw_with_players(ui, &[])
}

pub fn draw_with_players(ui: &mut egui::Ui, players: &[PlayerInfo]) -> Option<PlayerSelectionAction> {
    let mut action = None;
    
    ui.vertical_centered(|ui| {
        ui.add_space(50.0);
        ui.heading("Select Player");
        ui.add_space(30.0);
        
        if players.is_empty() {
            ui.label("No players found. Create a new player first.");
            ui.add_space(20.0);
        } else {
            egui::ScrollArea::vertical()
                .max_height(400.0)
                .show(ui, |ui| {
                    for player in players {
                        ui.add_space(5.0);
                        if ui.add_sized(
                            [400.0, 40.0],
                            egui::Button::new(&player.name)
                        ).clicked() {
                            action = Some(PlayerSelectionAction::SelectPlayer(player.id));
                        }
                    }
                });
            ui.add_space(20.0);
        }
        
        if ui.button("Back to Menu").clicked() {
            action = Some(PlayerSelectionAction::Back);
        }
    });
    
    action
}
