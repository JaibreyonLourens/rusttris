pub const CREATE_GAMES_TABLE: &str = "
    CREATE TABLE IF NOT EXISTS games (
        id INTEGER PRIMARY KEY,
        player_id INTEGER NOT NULL,
        score INTEGER NOT NULL,
        level INTEGER NOT NULL,
        lines_cleared INTEGER NOT NULL,
        singles INTEGER NOT NULL,
        doubles INTEGER NOT NULL,
        triples INTEGER NOT NULL,
        tetrises INTEGER NOT NULL,
        max_combo INTEGER NOT NULL,
        pieces_placed INTEGER NOT NULL,
        duration_seconds INTEGER NOT NULL,
        back_to_backs INTEGER NOT NULL,
        played_at INTEGER NOT NULL,
        FOREIGN KEY(player_id) REFERENCES players(id)
    );
";