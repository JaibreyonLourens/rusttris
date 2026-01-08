use eframe::egui;

use crate::enums::game_actions::GameAction;
use crate::classes::game_options::GameOptions;

pub enum OptionSelectionAction {
    Save,
    Cancel,
}

pub struct OptionsScreen {
    pub waiting_for_key: Option<GameAction>,
}

impl Default for OptionsScreen {
    fn default() -> Self {
        Self {
            waiting_for_key: None,
        }
    }
}

impl OptionsScreen {
    pub fn draw(&mut self, ui: &mut egui::Ui, options: &mut GameOptions) -> Option<OptionSelectionAction> {
        let mut action = None;
        
        // Check for key press if we're waiting for input
        if let Some(waiting_action) = &self.waiting_for_key {
            if let Some(pressed_key) = detect_key_press(ui) {
                // Assign the new key
                if let Some(key_binding) = options.key_bindings.get_mut(waiting_action) {
                    *key_binding = pressed_key;
                }
                self.waiting_for_key = None;
            }
            
            // Allow ESC to cancel rebinding
            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                self.waiting_for_key = None;
            }
        }
        
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            ui.heading("Options");
            ui.add_space(30.0);

            // Ghost piece opacity slider
            ui.horizontal(|ui| {
                ui.label("Ghost Piece Opacity:");
                let mut alpha = options.ghost_piece_alpha as f32 / 255.0;
                if ui.add(egui::Slider::new(&mut alpha, 0.0..=1.0).suffix(" %")).changed() {
                    options.ghost_piece_alpha = (alpha * 255.0) as u8;
                }
            });
            ui.add_space(10.0);

            // DAS delay slider
            ui.horizontal(|ui| {
                ui.label("DAS Delay:");
                ui.add(egui::Slider::new(&mut options.das_delay, 17.0..=333.0).suffix(" ms"));
            });
            ui.add_space(10.0);

            // ARR delay slider
            ui.horizontal(|ui| {
                ui.label("ARR Delay:");
                ui.add(egui::Slider::new(&mut options.arr_delay, 0.0..=83.0).suffix(" ms"));
            });
            ui.add_space(20.0);

            // Key bindings
            ui.label("Key Bindings:");
            ui.add_space(10.0);
            
            let actions = [
                GameAction::MoveLeft,
                GameAction::MoveRight,
                GameAction::SoftDrop,
                GameAction::HardDrop,
                GameAction::RotateCW,
                GameAction::RotateCCW,
                GameAction::HoldPiece,
                GameAction::PauseGame,
            ];
            
            for action_key in actions.iter() {
                ui.horizontal(|ui| {
                    ui.label(format!("{:?}:", action_key));
                    ui.add_space(10.0);
                    
                    let is_waiting = self.waiting_for_key.as_ref() == Some(action_key);
                    
                    if let Some(current_key) = options.key_bindings.get(action_key) {
                        let button_text = if is_waiting {
                            "Press any key...".to_string()
                        } else {
                            format!("{:?}", current_key)
                        };
                        
                        let button = ui.button(button_text);
                        
                        if button.clicked() && !is_waiting {
                            self.waiting_for_key = Some(*action_key);
                        }
                        
                        if is_waiting {
                            button.highlight();
                        }
                    }
                });
            }
            
            if self.waiting_for_key.is_some() {
                ui.add_space(10.0);
                ui.colored_label(egui::Color32::YELLOW, "Press ESC to cancel");
            }
            
            ui.add_space(30.0);

            // Save and Cancel buttons
            if ui.button("Save").clicked() {
                action = Some(OptionSelectionAction::Save);
            }
            ui.add_space(10.0);
            if ui.button("Cancel").clicked() {
                action = Some(OptionSelectionAction::Cancel);
            }
        });

        action
    }
}

fn detect_key_press(ui: &egui::Ui) -> Option<egui::Key> {
    ui.input(|i| {
        // Check all common keys
        for key in [
            egui::Key::ArrowUp,
            egui::Key::ArrowDown,
            egui::Key::ArrowLeft,
            egui::Key::ArrowRight,
            egui::Key::Space,
            egui::Key::A, egui::Key::B, egui::Key::C, egui::Key::D,
            egui::Key::E, egui::Key::F, egui::Key::G, egui::Key::H,
            egui::Key::I, egui::Key::J, egui::Key::K, egui::Key::L,
            egui::Key::M, egui::Key::N, egui::Key::O, egui::Key::P,
            egui::Key::Q, egui::Key::R, egui::Key::S, egui::Key::T,
            egui::Key::U, egui::Key::V, egui::Key::W, egui::Key::X,
            egui::Key::Y, egui::Key::Z,
            egui::Key::Num0, egui::Key::Num1, egui::Key::Num2,
            egui::Key::Num3, egui::Key::Num4, egui::Key::Num5,
            egui::Key::Num6, egui::Key::Num7, egui::Key::Num8,
            egui::Key::Num9,
        ] {
            if i.key_pressed(key) {
                return Some(key);
            }
        }
        None
    })
}
