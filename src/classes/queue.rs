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
        // If pieces is empty, generate initial bags
        if self.pieces.is_empty() {
            self.generate_seven_bag();
        }

        // Remove and return the first piece from pieces
        let piece = if !self.pieces.is_empty() {
            Some(self.pieces.remove(0))
        } else {
            None
        };

        // Refill pieces from next_pieces to keep it at 7
        if self.next_pieces.is_empty() {
            // Generate a new next bag if it's empty
            self.generate_new_next_bag();
        }
        
        // Now move one piece from next_pieces to pieces (guaranteed to have items now)
        if !self.next_pieces.is_empty() {
            let next_piece = self.next_pieces.remove(0);
            self.pieces.push(next_piece);
        }

        piece
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