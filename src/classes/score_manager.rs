use crate::enums::t_spin_type::TSpinType;

pub struct ScoreManager {
    pub score: u32,
    back_to_back: u32,
}

impl ScoreManager {
    pub fn new() -> Self {
        Self { score: 0, back_to_back: 0 }
    }
    
    pub fn lines_cleared(&mut self, lines_cleared: u32, level:u32, combo: u32, t_spin: TSpinType, is_all_clear: bool){
        
        let base_score = match (t_spin, lines_cleared) {
            // T-Spin scores
            (TSpinType::Full, 0) => 400,      // T-Spin no lines
            (TSpinType::Full, 1) => 800,      // T-Spin Single
            (TSpinType::Full, 2) => 1200,     // T-Spin Double
            (TSpinType::Full, 3) => 1600,     // T-Spin Triple
            (TSpinType::Mini, 1) => 200,      // T-Spin Mini Single
            (TSpinType::Mini, 2) => 400,      // T-Spin Mini Double
            
            // Regular line clears
            (TSpinType::None, 1) => 100,      // Single
            (TSpinType::None, 2) => 300,      // Double
            (TSpinType::None, 3) => 500,      // Triple
            (TSpinType::None, 4) => 800,      // Tetris
            
            _ => 0,
        };
        
        let mut total_score = base_score * level;

        // Apply back-to-back bonus for difficult clears (Tetris or T-Spins)
        let is_difficult = lines_cleared == 4 || t_spin != TSpinType::None;
        if self.back_to_back > 0 && is_difficult {
            total_score += total_score / 2;  // 1.5x multiplier for back-to-back
        }
        
        // Update back-to-back counter
        if is_difficult {
            self.back_to_back += 1;
        } else {
            self.back_to_back = 0;
        }
        
        // Apply combo bonus
        if combo > 1 {
            total_score += (combo - 1) * 50 * level;
        }
        
        // Apply All Clear bonus
        if is_all_clear {
            let all_clear_bonus = match (t_spin, lines_cleared) {
                (TSpinType::Full, _) => 2000,   // T-Spin All Clear
                (_, 1) => 800,                   // Single All Clear
                (_, 2) => 1200,                  // Double All Clear
                (_, 3) => 1800,                  // Triple All Clear
                (_, 4) => 2000,                  // Tetris All Clear
                _ => 0,
            };
            total_score += all_clear_bonus * level;
        }
        
        self.score += total_score;
    }
    
    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn drop(&mut self, drop_type: u32, cells_dropped: u32){
        self.score += (drop_type * cells_dropped) as u32;
    }


    
}