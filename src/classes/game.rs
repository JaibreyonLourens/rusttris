
use super::board::Board;
use super::piece::Piece;
use super::queue::Queue;
use super::score_manager::ScoreManager;
use super::hold::HoldQueue;
use crate::enums::states::GameState;

pub struct Game {
    pub board: Board,
    pub queue: Queue,
    pub hold_queue: HoldQueue,
    pub current_piece: Option<Piece>,
    level: u32,
    combo: u32,
    score_manager: ScoreManager,
    lines_cleared: u32,
    game_state: GameState,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            board: Board::new(),
            queue: Queue::new(),
            hold_queue: HoldQueue::new(),
            current_piece: None,
            level: 1,
            combo: 0,
            score_manager: ScoreManager::new(),
            lines_cleared: 0,
            game_state: GameState::Menu,
        };
        
        // Generate and test 7-bag
        game.queue.generate_seven_bag();
        game.current_piece = game.queue.get_next_piece();

    
        
        game
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        self.handle_input(ctx);
        // Game logic will go here
    }

    pub fn get_state(&self) -> GameState {
        self.game_state
    }

    pub fn get_score(&self) -> u32 {
        self.score_manager.get_score()
    }

    pub fn get_lines_cleared(&self) -> u32 {
        self.lines_cleared
    }

    pub fn draw_game_board(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Left side - the game board
            ui.vertical(|ui | {
                self.hold_queue.draw(ui);
            });
            ui.vertical(|ui| {
              
                // Temporarily draw current piece on board
                if let Some(piece) = &self.current_piece {
                    let blocks = piece.get_blocks();
                    let id = piece.get_id();
                    
                    for (row, col) in &blocks {
                        if *row >= 0 && *col >= 0 {
                            self.board.set_cell(*row as usize, *col as usize, id);
                        }
                    }
                }
                
                self.board.draw(ui);
                
                // Clear piece blocks after drawing
                if let Some(piece) = &self.current_piece {
                    let blocks = piece.get_blocks();
                    for (row, col) in &blocks {
                        if *row >= 0 && *col >= 0 {
                            self.board.set_cell(*row as usize, *col as usize, 0);
                        }
                    }
                }
            });
            
            ui.add_space(20.0);
            
            // Right side - the queue and stats
            ui.vertical(|ui| {
                // Display score and stats
                ui.heading("Game Stats");
                ui.add_space(5.0);
                ui.label(format!("Score: {}", self.score_manager.get_score()));
                ui.label(format!("Level: {}", self.level));
                ui.label(format!("Lines: {}", self.lines_cleared));
                ui.label(format!("Combo: {}", self.combo));
                
                ui.add_space(20.0);
                
                // Display next pieces
                self.queue.draw(ui, 5); // Show next 5 pieces
            });
        });
    }

    fn handle_input(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            // Pause/unpause
            if i.key_pressed(egui::Key::P) {
                match self.game_state {
                    GameState::Playing => self.game_state = GameState::Paused,
                    GameState::Paused => self.game_state = GameState::Playing,
                    _ => {}
                }
            }
            
            // Only handle game input when playing
            if self.game_state != GameState::Playing {
                return;
            }
            
            // Single key press (triggers once per press)
            if i.key_pressed(egui::Key::ArrowLeft) {
                if let Some(piece) = &mut self.current_piece {
                    piece.move_left();
                    // Check if valid, if not undo the move
                    if !self.board.is_valid_position(&piece.get_blocks()) {
                        piece.move_right(); // Undo
                    }
                }
            }
            
            if i.key_pressed(egui::Key::ArrowRight) {
                if let Some(piece) = &mut self.current_piece {
                    piece.move_right();
                    // Check if valid, if not undo the move
                    if !self.board.is_valid_position(&piece.get_blocks()) {
                        piece.move_left(); // Undo
                    }
                }
            }
            
            if i.key_pressed(egui::Key::ArrowUp) {
                // Rotate piece (to be implemented)
                if let Some(piece) = &mut self.current_piece {
                    piece.rotate_clockwise();
                    // Check if valid, if not undo the rotation
                    if !self.board.is_valid_position(&piece.get_blocks()) {
                        piece.rotate_counterclockwise(); // Undo
                    }
                }
            }

            if i.key_pressed(egui::Key::X) {
                // Rotate piece counterclockwise
                if let Some(piece) = &mut self.current_piece {
                    piece.rotate_counterclockwise();
                    // Check if valid, if not undo the rotation
                    if !self.board.is_valid_position(&piece.get_blocks()) {
                        piece.rotate_clockwise(); // Undo
                    }
                }
            }
            
            if i.key_pressed(egui::Key::ArrowDown) {
                if let Some(piece) = &mut self.current_piece {
                    piece.move_down();
                    // Check if valid, if not undo the move and lock the piece
                    self.score_manager.drop(1, 1);
                    if !self.board.is_valid_position(&piece.get_blocks()) {
                        piece.move_up(); // Undo
                        self.lock_piece();
                    }

                }
            }
            if i.key_pressed(egui::Key::C) {
                // Hold piece
                self.hold_queue.hold_piece(&mut self.current_piece, &mut self.queue);
            }
            
            if i.key_pressed(egui::Key::Space) {
                // Hard drop
                self.hard_drop();
            }
        });
    }

    fn hard_drop(&mut self) {
        if let Some(piece) = &mut self.current_piece {
            let mut cells_dropped = 0;
            
            // Move piece down until it collides
            loop {
                piece.move_down();
                if !self.board.is_valid_position(&piece.get_blocks()) {
                    piece.move_up(); // Undo last move
                    break;
                }
                cells_dropped += 1;
            }
            
            // Award hard drop points (2 points per cell)
            if cells_dropped > 0 {
                self.score_manager.drop(2, cells_dropped);
            }
        }
        
        // Lock the piece immediately
        self.lock_piece();
    }

    fn lock_piece(&mut self) {
        if let Some(piece) = &self.current_piece {
            let blocks = piece.get_blocks();
            let id = piece.get_id();
            
            // Place piece blocks permanently on the board
            for (row, col) in &blocks {
                if *row >= 0 && *col >= 0 {
                    self.board.set_cell(*row as usize, *col as usize, id);
                }
            }
        }
        
        // Clear completed lines
        let cleared = self.board.clear_lines();
        if cleared > 0 {
            // Increment combo
            self.combo += 1;
            
            // Use score manager to handle scoring
            self.score_manager.lines_cleared(cleared, self.level, self.combo);
            self.lines_cleared += cleared;
            
            println!("Cleared {} line(s)! Combo: {} Total lines: {}", cleared, self.combo, self.lines_cleared);
            println!("Score: {}", self.score_manager.get_score());
        } else {
            // Reset combo if no lines cleared
            self.combo = 0;
        }
        
        // Reset hold permission after locking
        self.hold_queue.reset_hold();
        
        // Spawn next piece
        self.spawn_next_piece();
    }

    fn spawn_next_piece(&mut self) {
        self.current_piece = self.queue.get_next_piece();
        
        if let Some(piece) = &self.current_piece {
            // Check if the new piece collides immediately (game over condition)
            if !self.board.is_valid_position(&piece.get_blocks()) {
                println!("Game Over! No space for new piece.");
                self.current_piece = None; // Clear the piece that couldn't spawn
                self.game_state = GameState::GameOver;
            }
        }
    }
    
    pub fn start_game(&mut self) {
        self.game_state = GameState::Playing;
        self.queue.generate_seven_bag();
        self.current_piece = self.queue.get_next_piece();
    }
    
    pub fn reset_game(&mut self) {
        *self = Self::new();
        self.game_state = GameState::Playing;
        self.queue.generate_seven_bag();
        self.current_piece = self.queue.get_next_piece();
    }
}