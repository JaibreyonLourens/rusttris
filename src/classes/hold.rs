use super::piece::Piece;


pub struct HoldQueue {
    pub held_piece: Option<Piece>,
    can_hold: bool,
}

impl HoldQueue {
    pub fn new() -> Self {
        Self {
            held_piece: None,
            can_hold: true,
        }
    }

    pub fn hold_piece(&mut self, current_piece: &mut Option<Piece>, queue: &mut super::queue::Queue) {
        if !self.can_hold {
            return; // Cannot hold again until piece is locked
        }

        if let Some(held) = &self.held_piece {
            // Swap held piece with current piece
            let temp = current_piece.take();
            *current_piece = Some((*held).clone());
            self.held_piece = temp;
        } else {
            // Move current piece to hold and get next piece from queue
            self.held_piece = current_piece.take();
            *current_piece = queue.get_next_piece();
        }

        self.can_hold = false; // Disable further holds until piece is locked
    }

    pub fn reset_hold(&mut self) {
        self.can_hold = true;
    }

    pub fn draw(&self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Hold");
            ui.add_space(10.0);
            
            // Allocate fixed size area for consistency
            let fixed_width = 100.0;
            let fixed_height = 100.0;
            
            ui.allocate_ui_with_layout(
                egui::vec2(fixed_width, fixed_height),
                egui::Layout::centered_and_justified(egui::Direction::TopDown),
                |ui| {
                    if let Some(piece) = &self.held_piece {
                        // Draw the held piece preview centered in the fixed area
                        piece.draw_preview(ui, 20.0);
                    } else {
                        ui.label("Empty");
                    }
                },
            );
        });
    }
}