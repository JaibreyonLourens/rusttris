use eframe::egui;

pub fn draw(ui: &mut egui::Ui, score: u32, lines_cleared: u32) -> bool {
    let mut should_restart = false;
    
    // Draw overlay background
    let screen_rect = ui.ctx().screen_rect();
    ui.painter().rect_filled(
        screen_rect,
        0.0,
        egui::Color32::from_black_alpha(180),
    );
    
    // Center the game over message
    egui::Area::new("game_over_overlay".into())
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ui.ctx(), |ui| {
            egui::Frame::none()
                .fill(egui::Color32::from_gray(40))
                .stroke(egui::Stroke::new(2.0, egui::Color32::WHITE))
                .inner_margin(30.0)
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("GAME OVER");
                        ui.add_space(20.0);
                        ui.label(format!("Final Score: {}", score));
                        ui.label(format!("Lines Cleared: {}", lines_cleared));
                        ui.add_space(20.0);
                        
                        if ui.button("Play Again").clicked() {
                            should_restart = true;
                        }
                    });
                });
        });
    
    should_restart
}

