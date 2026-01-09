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
        quadruples INTEGER NOT NULL,
        max_combo INTEGER NOT NULL,
        pieces_placed INTEGER NOT NULL,
        duration_seconds INTEGER NOT NULL,
        back_to_backs INTEGER NOT NULL,
        t_spin_singles INTEGER DEFAULT 0,
        t_spin_doubles INTEGER DEFAULT 0,
        t_spin_triples INTEGER DEFAULT 0,
        all_clears INTEGER DEFAULT 0,
        played_at INTEGER NOT NULL,
        FOREIGN KEY(player_id) REFERENCES players(id)
    );
";