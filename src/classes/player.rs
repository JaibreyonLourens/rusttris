use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStats {
    pub score: u32,
    pub lines_cleared: u32,
    pub level_reached: u32,
    pub pieces_placed: u32,
    pub singles: u32,
    pub doubles: u32,
    pub triples: u32,
    pub quadruples: u32,
    pub max_combo: u32,
    pub back_to_backs: u32,
    pub t_spins_singles: u32,
    pub t_spins_doubles: u32,
    pub t_spins_triples: u32,
    pub all_clears: u32,
    pub duration_seconds: u64,
    pub timestamp: u64,
}

impl GameStats {
    pub fn new() -> Self {
        Self {
            score: 0,
            lines_cleared: 0,
            level_reached: 1,
            pieces_placed: 0,
            singles: 0,
            doubles: 0,
            triples: 0,
            quadruples: 0,
            max_combo: 0,
            back_to_backs: 0,
            t_spins_singles: 0,
            t_spins_doubles: 0,
            t_spins_triples: 0,
            all_clears: 0,
            duration_seconds: 0,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    pub total_games: u32,
    pub total_score: u64,
    pub total_lines: u64,
    pub total_pieces: u64,
    pub total_singles: u64,
    pub total_doubles: u64,
    pub total_triples: u64,
    pub total_quadruples: u64,
    pub total_back_to_backs: u64,
    pub total_t_spins: u64,
    pub total_t_spin_singles: u64,
    pub total_t_spin_doubles: u64,
    pub total_t_spin_triples: u64,

    pub total_all_clears: u64,
    pub highest_score: u32,
    pub highest_level: u32,
    pub longest_combo: u32,
    pub total_playtime_seconds: u64,
    pub game_history: Vec<GameStats>,
}

impl PlayerStats {
    pub fn new() -> Self {
        Self {
            total_games: 0,
            total_score: 0,
            total_lines: 0,
            total_pieces: 0,
            total_singles: 0,
            total_doubles: 0,
            total_triples: 0,
            total_quadruples: 0,
            total_back_to_backs: 0,
            total_t_spins: 0,
            total_t_spin_singles: 0,
            total_t_spin_doubles: 0,
            total_t_spin_triples: 0,
            total_all_clears: 0,
            highest_score: 0,
            highest_level: 0,
            longest_combo: 0,
            total_playtime_seconds: 0,
            game_history: Vec::new(),
        }
    }

    pub fn add_game(&mut self, game_stats: GameStats) {
        self.total_games += 1;
        self.total_score += game_stats.score as u64;
        self.total_lines += game_stats.lines_cleared as u64;
        self.total_pieces += game_stats.pieces_placed as u64;
        self.total_singles += game_stats.singles as u64;
        self.total_doubles += game_stats.doubles as u64;
        self.total_triples += game_stats.triples as u64;
        self.total_quadruples += game_stats.quadruples as u64;
        self.total_back_to_backs += game_stats.back_to_backs as u64;
        self.total_t_spin_singles += game_stats.t_spins_singles as u64;
        self.total_t_spin_doubles += game_stats.t_spins_doubles as u64;
        self.total_t_spin_triples += game_stats.t_spins_triples as u64;
        self.total_t_spins += (game_stats.t_spins_singles + game_stats.t_spins_doubles + game_stats.t_spins_triples) as u64;
        self.total_all_clears += game_stats.all_clears as u64;
        self.total_playtime_seconds += game_stats.duration_seconds;

        // Update personal bests
        if game_stats.score > self.highest_score {
            self.highest_score = game_stats.score;
        }
        if game_stats.level_reached > self.highest_level {
            self.highest_level = game_stats.level_reached;
        }
        if game_stats.max_combo > self.longest_combo {
            self.longest_combo = game_stats.max_combo;
        }

        // Keep last 100 games in history
        self.game_history.push(game_stats);
        if self.game_history.len() > 100 {
            self.game_history.remove(0);
        }
    }

    pub fn get_average_score(&self) -> f64 {
        if self.total_games == 0 {
            0.0
        } else {
            self.total_score as f64 / self.total_games as f64
        }
    }

    pub fn get_average_lines(&self) -> f64 {
        if self.total_games == 0 {
            0.0
        } else {
            self.total_lines as f64 / self.total_games as f64
        }
    }

    pub fn get_recent_games(&self, count: usize) -> &[GameStats] {
        let start = self.game_history.len().saturating_sub(count);
        &self.game_history[start..]
    }
}

pub struct Player {
    pub id: Option<i64>,
    pub name: String,
    pub stats: PlayerStats,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            id: None,
            name,
            stats: PlayerStats::new(),
        }
    }

    pub fn finish_game(&mut self, game_stats: GameStats) {
        println!("Game finished! Score: {}, Lines: {}, Level: {}", 
                 game_stats.score, game_stats.lines_cleared, game_stats.level_reached);
        self.stats.add_game(game_stats);
    }

    // Serialization methods for future SQLite/file storage
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self.stats).unwrap_or_default()
    }

    pub fn from_json(name: String, json: &str) -> Self {
        let stats = serde_json::from_str(json).unwrap_or_else(|_| PlayerStats::new());
        Self { id: None, name, stats }
    }
}
