const B2B_SCORE: u32 = 1200;
pub struct ScoreManager {
    pub score: u32,
    back_to_back: u32,
}

impl ScoreManager {
    pub fn new() -> Self {
        Self { score: 0, back_to_back: 0 }
    }
    
    pub fn lines_cleared(&mut self, lines_cleared: u32, level:u32, combo: u32,){
        let base_score = match lines_cleared {
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800,
            _ => 0,
        };
        
        let mut total_score = base_score * level;

        // Apply back-to-back bonus
        if self.back_to_back > 0 && lines_cleared == 4 {
            total_score = B2B_SCORE * level;
        }
        
        // Update back-to-back counter
        if lines_cleared == 4 {
            self.back_to_back += 1;
        } else {
            self.back_to_back = 0;
        }
        // Apply combo bonus
        if combo > 1 {
            total_score += combo * 50 * level;
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