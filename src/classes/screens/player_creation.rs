use eframe::egui;

pub fn draw(ui: &mut egui::Ui, player_name: &mut String, error_message: Option<&str>) -> bool {
    let mut should_create = false;
    
    ui.vertical_centered(|ui| {
        ui.add_space(100.0);
        ui.heading("Welcome to Rusttris!");
        ui.add_space(50.0);
        
        ui.label("Enter your name:");
        ui.add_space(10.0);
        
        let response = ui.text_edit_singleline(player_name);
        
        // Auto-focus the text input
        if response.gained_focus() || player_name.is_empty() {
            response.request_focus();
        }
        
        // Show error message if present
        if let Some(error) = error_message {
            ui.add_space(10.0);
            ui.colored_label(egui::Color32::RED, error);
        }
        
        ui.add_space(20.0);
        
        let button_enabled = !player_name.trim().is_empty();
        
        if ui.add_enabled(button_enabled, egui::Button::new("Create Player")).clicked() {
            should_create = true;
        }
        
        // Also allow Enter key to submit
        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) && button_enabled {
            should_create = true;
        }
    });
    
    should_create
}