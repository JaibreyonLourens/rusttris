# Rusttris - Tetris in Rust

A modern Tetris clone built with Rust using the eframe/egui GUI framework with SQLite persistence.

## About

Rusttris is a fully-featured Tetris implementation that follows the official Tetris guidelines, including advanced mechanics like the 7-bag randomization system, hold queue, ghost pieces, gravity system, lock delay, and comprehensive player statistics tracking.

### Disclaimer
This project is for educational purposes only and is not affiliated with The Tetris Company and is not intended for commercial use.

## Current Features

### Game Mechanics
- **7-Bag Randomization System**: Ensures fair piece distribution using the modern Tetris standard
- **Hold Queue**: Store a piece for later use (C key)
- **Ghost Pieces**: Semi-transparent preview showing where the current piece will land
- **DAS (Delayed Auto Shift)**: Modern Tetris-style piece movement with 133ms delay and 33ms auto-repeat rate
- **Gravity System**: Automatic piece descent with exponential speed increase based on level (1.0s → 0.1s)
- **Lock Delay**: Grace period (500ms → 100ms based on level) before piece locks, resets on movement/rotation
- **Piece Movement**: Smooth horizontal movement with DAS auto-repeat and collision detection
- **Piece Rotation**: Clockwise and counterclockwise rotation with wall kicks
- **Hard Drop**: Instantly drop pieces to the bottom (Space)
- **Soft Drop**: Speed up piece descent for bonus points (Down arrow)
- **Collision Detection**: Basic detection for board boundaries and piece overlap

### Scoring System
- Line clear scoring (Single: 100, Double: 300, Triple: 500, Tetris: 800)
- Combo bonuses for consecutive line clears
- Back-to-back Tetris bonuses (+1200 points)
- Soft drop and hard drop points
- Level progression based on lines cleared

### Player Management & Statistics
- **Player Creation**: Create and manage multiple players
- **Player Selection**: Switch between different player profiles
- **SQLite Database**: Persistent storage of player data and game history
- **Comprehensive Stats Tracking**:
  - Per-game statistics (score, lines, level, piece counts, combos, duration)
  - All-time player statistics (total games, highest scores, averages, totals)
  - Detailed breakdown: singles, doubles, triples, quadruples, back-to-backs
  - Game history (last 100 games per player)
  - Playtime tracking
- **Auto-save**: Games automatically saved to database on completion
- **Last Player Loading**: Automatically loads the most recently active player on startup

### User Interface
- **Player Creation Screen**: First-time setup for new players with text input and duplicate name validation
- **Player Selection Screen**: Scrollable list of all players
- **Leaderboard Screen**: Multi-category rankings (High Score, Highest Level, Most Lines, Max Combo, Most Tetrises, Back-to-Backs) with top 10 players
- **Options Screen**: Configurable settings with live preview
  - **Key Rebinding**: Click-to-rebind interface for all game actions with visual feedback
  - **DAS Timing**: Adjustable delay (17-333ms) for auto-shift activation
  - **ARR Timing**: Adjustable auto-repeat rate (0-83ms) for piece movement speed
  - **Ghost Piece Opacity**: Slider to control transparency of ghost piece
  - **Settings Persistence**: All settings saved to `settings.json` and loaded on startup
- **Menu Screen**: Main menu showing current player with game options
- **Game Screen**: Clean layout showing:
  - Hold queue (left)
  - Game board with ghost piece preview (center)
  - Next 5 pieces preview (right)
  - Score, level, lines cleared, and combo counter
- **Pause Screen**: Overlay when paused (P key)
- **Game Over Screen**: Display final score with statistics and restart option

### Technical Architecture
- **Modular Design**: Separated concerns with dedicated modules
  - `Game`: Core game logic with DAS, gravity, and lock delay systems
  - `Board`: 10x22 playfield management with overlay support
  - `Piece`: Tetromino shapes, rotations, and transparency rendering
  - `Queue`: 7-bag piece generation
  - `HoldQueue`: Hold functionality
  - `ScoreManager`: Scoring calculations
  - `ScreenManager`: Screen rendering orchestration with state management
  - `Player`: Player identity and statistics aggregation
  - `DbManager`: SQLite database operations (CRUD for players and games, leaderboard queries)
  - `screens/`: Individual screen implementations (menu, paused, game_over, player_creation, player_selection, leaderboard, options)
  - `database/schemas/`: SQL schema definitions (players, games tables)
- **GameOptions**: Centralized settings management with serialization
  - Custom key bindings stored as HashMap<GameAction, egui::Key>
  - DAS/ARR timing configuration
  - JSON persistence with custom serialization for egui types
- **Repository Pattern**: Database layer separated from domain logic
- **State Management**: GameState enum for screen transitions (PlayerCreation, PlayerSelection, Menu, Playing, Paused, GameOver, Leaderboard, Options)

## Controls

Default controls (customizable in Options menu):

- **Arrow Keys**: Move piece left/right
- **Up Arrow / X**: Rotate clockwise
- **Z**: Rotate counterclockwise
- **Down Arrow**: Soft drop
- **Space**: Hard drop
- **C**: Hold piece
- **P**: Pause/unpause game

All key bindings can be changed in the Options screen accessible from the main menu.

## Database Schema

### Players Table
- `id`: Primary key (auto-increment)
- `name`: Unique player name
- `created_at`: Unix timestamp

### Games Table
- `id`: Primary key (auto-increment)
- `player_id`: Foreign key to players
- `score`, `level`, `lines_cleared`: Game metrics
- `singles`, `doubles`, `triples`, `quadruples`: Line clear breakdown
- `max_combo`, `back_to_backs`: Bonus stats
- `pieces_placed`, `duration_seconds`: Gameplay stats
- `pCurrent Development Focus
- **SRS (Super Rotation System)**: Implementing the official Tetris rotation system with wall kicks
- **UI Improvements**: Enhanced visual feedback and polish

### Planned Features
- **Statistics Display Screen**: Detailed view of individual player stats and game history
- **Sound Effects**: Audio feedback for actions and line clears
- **Visual Effects**: Animations for line clears and level ups
- **Statistics Display Screen**: Detailed view of individual player stats and game history
- **Sound Effects**: Audio feedback for actions and line clears
- **Visual Effects**: Animations for line clears and level ups
- **Settings Screen**: Customizable DAS/ARR timing, ghost piece transparency, controls remapping
- **T-Spin Detection**: Advanced scoring for T-spin maneuvers
- **Perfect Clear Bonus**: Extra points for clearing the entire board

## Building and Running

### Prerequisites
- Rust (latest stable version)
- Cargo

### Run the Game
```bash
cargo run --release
```

### Development Build
```bash
cargo run
```and settings file `settings.json` are

### Database Location
The SQLite database file `rusttris.db` is created in the project root directory.

## Project Structure

```
rusttris/
├── src/
│   ├── main.rs                    # Application entry point
│   ├── classes/
│   │   ├── board.rs              # Game board logic with overlay support
│   │   ├── game.rs               # Core game mechanics (gravity, lock delay)
│   │   ├── piece.rs              # Tetromino pieces with transparency
│   │   ├── queue.rs              # 7-bag randomization
│   │   ├── hold.rs               # Hold queue
│   │   ├── score_manager.rs      # Scoring system
│   │   ├── screen_manager.rs     # Screen orchestration
│   │   ├── player.rs             # Player and statistics structs
│   │   ├── game_options.rs       # Settings and key bindings
│   │   ├── database/
│   │   │   ├── database.rs       # DbManager with CRUD operations
│   │   │   └── schemas/
│   │   │       ├── players.rs    # Players table schema
│   │   │       └── games.rs      # Games table schema
│   │   └── screens/              # Individual screens
│   │       ├── menu.rs
│   │       ├── paused.rs
│   │       ├── game_over.rs
│   │       ├── player_selection.rs
│   │       ├── leaderboard.rs
│   │       └── optionsrs
│   │       └── player_selection.rs
│   └── enums/
├── settings.json                 # User settings (generated)
│       └── states.rs             # Game states
├── Cargo.toml
├── rusttris.db                   # SQLite database (generated)
└── README.md
```

## Dependencies

- **eframe** (0.29): Cross-platform GUI framework
- **egui** (0.29): Immediate mode GUI library
- **rand** (0.8): Random number generation for piece shuffling
- **rusqlite** (0.38): SQLite database interface
- **serde** (1.0): Serialization framework
- **serde_json** (1.0): JSON serialization

## License

This project is open source and available for educational purposes.

---
SRS (Super Rotation System) implementation and UI improvements
**Status**: Active Development

Current focus: Advanced mechanics (T-spins, perfect clears) and visual/audio polish
