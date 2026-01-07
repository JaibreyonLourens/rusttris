use rusqlite::{params, Connection, Result};
use crate::classes::player::{GameStats};
use crate::classes::player::Player;
use crate::classes::database::schemas::{players, games};    
use std::time::{SystemTime, UNIX_EPOCH};
pub struct DbManager {
    // Database connection and related fields
    conn: Connection
}

impl DbManager {
    pub fn new() -> Self {
        // Initialize database connection
        let db_manager = DbManager {
            conn: Connection::open("rusttris.db").unwrap(),
        };
        db_manager.init_schemas();
        db_manager
    }

    pub fn init_schemas(&self) {
        // Create tables for players and games if they don't exist
        self.conn.execute_batch(players::CREATE_PLAYERS_TABLE).unwrap();
        self.conn.execute_batch(games::CREATE_GAMES_TABLE).unwrap();
    }

    pub fn get_connection(&self) -> &Connection {
        &self.conn
    }

    pub fn create_player(&self, name: &str) -> Result<i64> {
        let now = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs() as i64;
        self.conn.execute(
            "INSERT INTO players (name, created_at) VALUES (?1, ?2)",
            params![name, now],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_player_id(&self, name: &str) -> Result<Option<i64>> {
        let mut stmt = self.conn.prepare("SELECT id FROM players WHERE name = ?1")?;
        let mut rows = stmt.query(params![name])?;

        if let Some(row) = rows.next()? {
            let id: i64 = row.get(0)?;
            Ok(Some(id))
        } else {
            Ok(None)
        }
    }
    pub fn get_player(&self, id: i64) -> Result<Option<Player>> {
        let mut stmt = self.conn.prepare("SELECT name FROM players WHERE id = ?1")?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            let name: String = row.get(0)?;
            Ok(Some(Player::new(name)))
        } else {
            Ok(None)
        }
    }

    pub fn save_game(&self, player_id: i64, game_stats: &GameStats) -> Result<()> {
        let now = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs() as i64;
        
        self.conn.execute(
            "INSERT INTO games (player_id, score, level, lines_cleared, singles, doubles, triples, quadruples, max_combo, pieces_placed, duration_seconds, back_to_backs, played_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                player_id,
                game_stats.score as i64,
                game_stats.level_reached as i64,
                game_stats.lines_cleared as i64,
                game_stats.singles as i64,
                game_stats.doubles as i64,
                game_stats.triples as i64,
                game_stats.quadruples as i64,
                game_stats.max_combo as i64,
                game_stats.pieces_placed as i64,
                game_stats.duration_seconds as i64,
                game_stats.back_to_backs as i64,
                now
            ],
        )?;
        Ok(())
    }

    pub fn get_last_active_player(&self) -> Result<Option<Player>> {
        // First try to get the player_id from the most recent game
        let mut stmt = self.conn.prepare(
            "SELECT player_id FROM games ORDER BY played_at DESC LIMIT 1"
        )?;
        let mut rows = stmt.query(params![])?;

        if let Some(row) = rows.next()? {
            let player_id: i64 = row.get(0)?;
            // Load the full player with their stats
            self.get_player_with_stats(player_id)
        } else {
            // No games played yet, get the most recently created player
            let mut stmt = self.conn.prepare(
                "SELECT id FROM players ORDER BY created_at DESC LIMIT 1"
            )?;
            let mut rows = stmt.query(params![])?;
            
            if let Some(row) = rows.next()? {
                let player_id: i64 = row.get(0)?;
                self.get_player_with_stats(player_id)
            } else {
                // No players exist at all
                Ok(None)
            }
        }
    }

    pub fn get_player_with_stats(&self, player_id: i64) -> Result<Option<Player>> {
        // Get player name
        let mut stmt = self.conn.prepare("SELECT name FROM players WHERE id = ?1")?;
        let mut rows = stmt.query(params![player_id])?;

        if let Some(row) = rows.next()? {
            let name: String = row.get(0)?;
            let mut player = Player::new(name);
            player.id = Some(player_id);

            // Load all games for this player and build stats
            self.load_player_stats(&mut player)?;

            Ok(Some(player))
        } else {
            Ok(None)
        }
    }

    fn load_player_stats(&self, player: &mut Player) -> Result<()> {
        let player_id = player.id.unwrap();
        
        let mut stmt = self.conn.prepare(
            "SELECT score, level, lines_cleared, singles, doubles, triples, quadruples, 
                    max_combo, pieces_placed, duration_seconds, played_at, back_to_backs
             FROM games 
             WHERE player_id = ?1 
             ORDER BY played_at ASC"
        )?;
        
        let game_iter = stmt.query_map(params![player_id], |row| {
            Ok(GameStats {
                score: row.get::<_, i64>(0)? as u32,
                lines_cleared: row.get::<_, i64>(2)? as u32,
                level_reached: row.get::<_, i64>(1)? as u32,
                singles: row.get::<_, i64>(3)? as u32,
                doubles: row.get::<_, i64>(4)? as u32,
                triples: row.get::<_, i64>(5)? as u32,
                quadruples: row.get::<_, i64>(6)? as u32,
                max_combo: row.get::<_, i64>(7)? as u32,
                pieces_placed: row.get::<_, i64>(8)? as u32,
                duration_seconds: row.get::<_, i64>(9)? as u64,
                timestamp: row.get::<_, i64>(10)? as u64,
                back_to_backs: row.get::<_, i64>(11).unwrap_or(0) as u32,
            })
        })?;

        // Add each game to player stats
        for game in game_iter {
            player.stats.add_game(game?);
        }

        Ok(())
    }

    pub fn get_all_players(&self) -> Result<Vec<(i64, String)>> {
        let mut stmt = self.conn.prepare("SELECT id, name FROM players ORDER BY name ASC")?;
        let player_iter = stmt.query_map(params![], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?;

        let mut players = Vec::new();
        for player in player_iter {
            players.push(player?);
        }

        Ok(players)
    }

    pub fn get_leaderboard(&self, limit: usize, category: LeaderboardCategory) -> Result<Vec<(String, u32)>> {
        let (column, alias) = match category {
            LeaderboardCategory::HighScore => ("MAX(g.score)", "value"),
            LeaderboardCategory::HighestLevel => ("MAX(g.level)", "value"),
            LeaderboardCategory::MostLines => ("MAX(g.lines_cleared)", "value"),
            LeaderboardCategory::MaxCombo => ("MAX(g.max_combo)", "value"),
            LeaderboardCategory::Mostquadruples => ("MAX(g.quadruples)", "value"),
            LeaderboardCategory::MostBackToBacks => ("MAX(g.back_to_backs)", "value"),
        };

        let query = format!(
            "SELECT p.name, {} as {}
             FROM players p
             JOIN games g ON p.id = g.player_id
             GROUP BY p.id
             ORDER BY {} DESC
             LIMIT ?1",
            column, alias, alias
        );

        let mut stmt = self.conn.prepare(&query)?;
        let leaderboard_iter = stmt.query_map(params![limit as i64], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?;

        let mut leaderboard = Vec::new();
        for entry in leaderboard_iter {
            leaderboard.push(entry?);
        }

        Ok(leaderboard)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LeaderboardCategory {
    HighScore,
    HighestLevel,
    MostLines,
    MaxCombo,
    Mostquadruples,
    MostBackToBacks,
}