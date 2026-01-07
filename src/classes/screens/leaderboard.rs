use eframe::egui;
use crate::classes::database::database::LeaderboardCategory;

pub struct LeaderboardState {
    pub current_category: LeaderboardCategory,
}

impl LeaderboardState {
    pub fn new() -> Self {
        Self {
            current_category: LeaderboardCategory::HighScore,
        }
    }
}

pub enum LeaderboardAction {
    Back,
}

pub fn draw(
    ui: &mut egui::Ui,
    state: &mut LeaderboardState,
    leaderboard_data: &[(String, u32)],
) -> Option<LeaderboardAction> {
    let mut action = None;

    ui.vertical_centered(|ui| {
        ui.add_space(30.0);
        ui.heading("🏆 Leaderboard");
        ui.add_space(20.0);

        // Category tabs
        ui.horizontal(|ui| {
            ui.add_space(20.0);
            
            if ui.selectable_label(
                matches!(state.current_category, LeaderboardCategory::HighScore),
                "High Score"
            ).clicked() {
                state.current_category = LeaderboardCategory::HighScore;
            }

            if ui.selectable_label(
                matches!(state.current_category, LeaderboardCategory::HighestLevel),
                "Highest Level"
            ).clicked() {
                state.current_category = LeaderboardCategory::HighestLevel;
            }

            if ui.selectable_label(
                matches!(state.current_category, LeaderboardCategory::MostLines),
                "Most Lines"
            ).clicked() {
                state.current_category = LeaderboardCategory::MostLines;
            }

            if ui.selectable_label(
                matches!(state.current_category, LeaderboardCategory::MaxCombo),
                "Max Combo"
            ).clicked() {
                state.current_category = LeaderboardCategory::MaxCombo;
            }

            if ui.selectable_label(
                matches!(state.current_category, LeaderboardCategory::Mostquadruples),
                "Most quadruples"
            ).clicked() {
                state.current_category = LeaderboardCategory::Mostquadruples;
            }

            if ui.selectable_label(
                matches!(state.current_category, LeaderboardCategory::MostBackToBacks),
                "Back-to-Backs"
            ).clicked() {
                state.current_category = LeaderboardCategory::MostBackToBacks;
            }
        });

        ui.add_space(20.0);

        // Category title
        let category_name = match state.current_category {
            LeaderboardCategory::HighScore => "High Scores",
            LeaderboardCategory::HighestLevel => "Highest Levels",
            LeaderboardCategory::MostLines => "Most Lines Cleared",
            LeaderboardCategory::MaxCombo => "Maximum Combos",
            LeaderboardCategory::Mostquadruples => "Most quadruples",
            LeaderboardCategory::MostBackToBacks => "Most Back-to-Backs",
        };
        ui.label(egui::RichText::new(category_name).size(18.0).strong());
        ui.add_space(15.0);

        // Leaderboard entries
        if leaderboard_data.is_empty() {
            ui.label("No data available yet. Play some games!");
        } else {
            egui::ScrollArea::vertical()
                .max_height(350.0)
                .show(ui, |ui| {
                    for (rank, (name, value)) in leaderboard_data.iter().enumerate() {
                        ui.horizontal(|ui| {
                            // Rank with medal icons for top 3
                            let rank_text = match rank {
                                0 => "1.",
                                1 => "2.",
                                2 => "3.",
                                _ => "",
                            };
                            
                            if rank < 3 {
                                ui.label(egui::RichText::new(rank_text).size(20.0));
                            } else {
                                ui.label(format!("{}.", rank + 1));
                            }
                            
                            ui.add_space(10.0);
                            
                            // Player name
                            ui.label(egui::RichText::new(name).size(16.0));
                            
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                // Value
                                ui.label(egui::RichText::new(format!("{}", value)).size(16.0).strong());
                            });
                        });
                        
                        ui.add_space(5.0);
                        ui.separator();
                        ui.add_space(5.0);
                    }
                });
        }

        ui.add_space(20.0);

        if ui.button("Back to Menu").clicked() {
            action = Some(LeaderboardAction::Back);
        }
    });

    action
}
