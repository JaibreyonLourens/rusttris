use eframe::egui;

pub fn draw(ui: &mut egui::Ui) {
    // Draw semi-transparent overlay
    let screen_rect = ui.ctx().screen_rect();
    ui.painter().rect_filled(
        screen_rect,
        0.0,
        egui::Color32::from_black_alpha(180),
    );
    
    // Center the pause message
    egui::Area::new("pause_overlay".into())
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ui.ctx(), |ui| {
            egui::Frame::none()
                .fill(egui::Color32::from_gray(40))
                .stroke(egui::Stroke::new(2.0, egui::Color32::WHITE))
                .inner_margin(30.0)
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("PAUSED");
                        ui.add_space(10.0);
                        ui.label("Press P to resume");
                    });
                });
        });
}
