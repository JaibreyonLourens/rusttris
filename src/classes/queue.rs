use super::piece::Piece;
use eframe::egui;

pub struct Queue {
    pieces: Vec<Piece>,
    next_pieces: Vec<Piece>,
    bag: Vec<&'static str>,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            pieces: Vec::new(),
            next_pieces: Vec::new(),
            bag: vec!["I", "O", "T", "S", "Z", "J", "L"],
        }
    }

    pub fn generate_seven_bag(&mut self) {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        let mut shuffled_bag = self.bag.clone();
        shuffled_bag.shuffle(&mut rng);
        
        if self.pieces.is_empty() {
            for name in shuffled_bag.iter() {
                self.pieces.push(Piece::create_piece(name));
            }
            shuffled_bag.shuffle(&mut rng);
            for name in shuffled_bag.iter() {
                self.next_pieces.push(Piece::create_piece(name));
            }
        }
    }
    
    pub fn get_piece_names(&self) -> (Vec<String>, Vec<String>) {
        let current = self.pieces.iter().map(|p| p.get_name().to_string()).collect();
        let next = self.next_pieces.iter().map(|p| p.get_name().to_string()).collect();
        (current, next)
    }

    pub fn get_next_piece(&mut self) -> Option<Piece> {
        // If next_pieces is empty, refill it
        if self.next_pieces.is_empty() {
            self.generate_new_next_bag();
        }

        // Remove and return the first piece
        if !self.pieces.is_empty() {
            let piece = self.pieces.remove(0);
            // Add the first piece from next_pieces to pieces
            if !self.next_pieces.is_empty() {
                let next_piece = self.next_pieces.remove(0);
                self.pieces.push(next_piece);
            }
            Some(piece)
        } else {
            self.generate_seven_bag();
            self.get_next_piece()
        }
    }

    fn generate_new_next_bag(&mut self) {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        let mut shuffled_bag = self.bag.clone();
        shuffled_bag.shuffle(&mut rng);
        
        for name in shuffled_bag.iter() {
            self.next_pieces.push(Piece::create_piece(name));
        }
    }
    
    pub fn draw(&self, ui: &mut egui::Ui, num_pieces: usize) {
        ui.vertical(|ui| {
            ui.heading("Next Pieces");
            ui.add_space(10.0);
            
            // Draw the first `num_pieces` from the queue
            for (_i, piece) in self.pieces.iter().take(num_pieces).enumerate() {
                // Draw the piece preview
                piece.draw_preview(ui, 15.0);
                ui.add_space(5.0);
            }
        });
    }
}