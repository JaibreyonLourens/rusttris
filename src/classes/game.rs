
use super::board::Board;
use super::piece::Piece;
use super::queue::Queue;
use super::score_manager::ScoreManager;
use super::hold::HoldQueue;
use super::player::GameStats;
use crate::enums::states::GameState;
use std::time::{SystemTime, UNIX_EPOCH};

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
    drop_timer: f32,
    drop_interval: f32,
    current_game_stats: GameStats,
    game_start_time: u64,
    lock_delay_timer: f32,
    lock_delay_duration: f32,
    piece_on_ground: bool,
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
            drop_timer: 0.0,
            drop_interval: 1.0,
            current_game_stats: GameStats::new(),
            game_start_time: 0,
            lock_delay_timer: 0.0,
            lock_delay_duration: 1.0, // 1000ms at level 1
            piece_on_ground: false,
        };
        
        // Generate and test 7-bag
        game.queue.generate_seven_bag();
        game.current_piece = game.queue.get_next_piece();

    
        
        game
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        self.handle_input(ctx);
        
        // Only apply gravity when playing
        if self.game_state == GameState::Playing {
            // Get delta time from egui
            let delta_time = ctx.input(|i| i.stable_dt);
            
            // Update drop timer
            self.drop_timer += delta_time;
            
            // Check if it's time to drop the piece
            if self.drop_timer >= self.drop_interval {
                self.drop_timer = 0.0;
                self.apply_gravity();
            }
            
            // Handle lock delay when piece is on ground
            if self.piece_on_ground {
                self.lock_delay_timer += delta_time;
                
                if self.lock_delay_timer >= self.lock_delay_duration {
                    self.lock_piece();
                    self.piece_on_ground = false;
                    self.lock_delay_timer = 0.0;
                }
            }
        }
        
        // Request repaint for smooth animation
        ctx.request_repaint();
    }

    pub fn get_state(&self) -> GameState {
        self.game_state
    }

    pub fn set_state(&mut self, state: GameState) {
        self.game_state = state;
    }

    pub fn get_score(&self) -> u32 {
        self.score_manager.get_score()
    }

    pub fn get_lines_cleared(&self) -> u32 {
        self.lines_cleared
    }

    pub fn get_game_stats(&mut self) -> GameStats {
        // Update current stats before returning
        self.current_game_stats.score = self.score_manager.get_score();
        self.current_game_stats.lines_cleared = self.lines_cleared;
        self.current_game_stats.level_reached = self.level;
        
        // Calculate duration
        if self.game_start_time > 0 {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            self.current_game_stats.duration_seconds = now - self.game_start_time;
        }
        
        self.current_game_stats.clone()
    }

    pub fn draw_game_board(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Left side - the game board
            ui.vertical(|ui | {
                self.hold_queue.draw(ui);
            });
            ui.vertical(|ui| {
                // Calculate ghost piece position
                let mut ghost_blocks = Vec::new();
                if let Some(piece) = &self.current_piece {
                    let mut ghost_piece = piece.clone();
                    
                    // Drop ghost piece until it collides
                    loop {
                        ghost_piece.move_down();
                        if !self.board.is_valid_position(&ghost_piece.get_blocks()) {
                            ghost_piece.move_up(); // Undo last move
                            break;
                        }
                    }
                    
                    // Get ghost blocks with transparent color
                    let ghost_color = ghost_piece.get_color_with_alpha(60);
                    for (row, col) in ghost_piece.get_blocks() {
                        ghost_blocks.push((row, col, ghost_color));
                    }
                }
              
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
                
                // Draw board with ghost piece overlay
                self.board.draw_with_overlay(ui, &ghost_blocks);
                
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
                    } else {
                        // Successful move, reset lock delay
                        self.reset_lock_delay();
                    }
                }
            }
            
            if i.key_pressed(egui::Key::ArrowRight) {
                if let Some(piece) = &mut self.current_piece {
                    piece.move_right();
                    // Check if valid, if not undo the move
                    if !self.board.is_valid_position(&piece.get_blocks()) {
                        piece.move_left(); // Undo
                    } else {
                        // Successful move, reset lock delay
                        self.reset_lock_delay();
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
                    } else {
                        // Successful rotation, reset lock delay
                        self.reset_lock_delay();
                    }
                }
            }

            if i.key_pressed(egui::Key::Z) {
                // Rotate piece counterclockwise
                if let Some(piece) = &mut self.current_piece {
                    piece.rotate_counterclockwise();
                    // Check if valid, if not undo the rotation
                    if !self.board.is_valid_position(&piece.get_blocks()) {
                        piece.rotate_clockwise(); // Undo
                    } else {
                        // Successful rotation, reset lock delay
                        self.reset_lock_delay();
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
                        self.piece_on_ground = true; // Start lock delay
                    } else {
                        // Successful move, reset lock delay
                        self.reset_lock_delay();
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
        
        // Lock the piece immediately (bypass lock delay)
        self.piece_on_ground = false;
        self.lock_delay_timer = 0.0;
        self.lock_piece();
    }
    
    fn reset_lock_delay(&mut self) {
        // Reset lock delay timer when piece is moved/rotated successfully
        if self.piece_on_ground {
            self.lock_delay_timer = 0.0;
        }
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
            
            // Track pieces placed
            self.current_game_stats.pieces_placed += 1;
        }
        
        // Clear completed lines
        let cleared = self.board.clear_lines();
        if cleared > 0 {
            // Track line clears by type
            match cleared {
                1 => self.current_game_stats.singles += 1,
                2 => self.current_game_stats.doubles += 1,
                3 => self.current_game_stats.triples += 1,
                4 => self.current_game_stats.quadruples += 1,
                _ => {}
            }
            
            // Increment combo
            self.combo += 1;
            
            // Track max combo
            if self.combo > self.current_game_stats.max_combo {
                self.current_game_stats.max_combo = self.combo;
            }
            
            // Use score manager to handle scoring
            self.score_manager.lines_cleared(cleared, self.level, self.combo);
            self.lines_cleared += cleared;
            
            // Check for level up
            self.update_level();
            
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
    
    fn apply_gravity(&mut self) {
        if let Some(piece) = &mut self.current_piece {
            piece.move_down();
            
            // Check if the move is valid
            if !self.board.is_valid_position(&piece.get_blocks()) {
                piece.move_up(); // Undo the move
                self.piece_on_ground = true; // Piece is now resting on something
            } else {
                // Piece successfully moved down, reset lock delay
                self.piece_on_ground = false;
                self.lock_delay_timer = 0.0;
            }
        }
    }
    
    fn calculate_drop_interval(&self) -> f32 {
        // Decrease interval as level increases (faster falling)
        // Formula: (0.8 - ((level - 1) * 0.007))^(level - 1)
        // This creates a good curve similar to official Tetris
        let base_interval = 1.0;
        let level_multiplier = 0.9_f32.powi(self.level as i32 - 1);
        (base_interval * level_multiplier).max(0.1) // Minimum 0.1 seconds
    }
    
    fn calculate_lock_delay(&self) -> f32 {
        // Decrease lock delay as level increases
        // Start at 500ms, reduce to minimum 100ms at high levels
        let base_delay = 0.5;
        let level_multiplier = 0.92_f32.powi(self.level as i32 - 1);
        (base_delay * level_multiplier).max(0.1) // Minimum 100ms
    }
    
    fn update_level(&mut self) {
        // Increase level every 10 lines
        let new_level = (self.lines_cleared / 10) + 1;
        if new_level != self.level {
            self.level = new_level;
            self.drop_interval = self.calculate_drop_interval();
            self.lock_delay_duration = self.calculate_lock_delay();
            println!("Level up! Now at level {}", self.level);
        }
    }
    
    pub fn start_game(&mut self) {
        self.game_state = GameState::Playing;
        self.queue.generate_seven_bag();
        self.current_piece = self.queue.get_next_piece();
        self.drop_interval = self.calculate_drop_interval();
        self.lock_delay_duration = self.calculate_lock_delay();
        
        // Reset and start tracking game stats
        self.current_game_stats = GameStats::new();
        self.game_start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    pub fn reset_game(&mut self) {
        *self = Self::new();
        self.game_state = GameState::Playing;
        self.queue.generate_seven_bag();
        self.current_piece = self.queue.get_next_piece();
        self.drop_interval = self.calculate_drop_interval();
        self.lock_delay_duration = self.calculate_lock_delay();
        
        // Reset and start tracking game stats
        self.current_game_stats = GameStats::new();
        self.game_start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}