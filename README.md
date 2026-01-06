# Rusttris - Tetris in Rust

A modern Tetris clone built with Rust using the eframe/egui GUI framework.

## About

Rusttris is a fully-featured Tetris implementation that follows the official Tetris guidelines, including advanced mechanics like the 7-bag randomization system, hold queue, and combo scoring.
### Disclaimer
This project is for educational purposes only and is not affiliated with The Tetris Company. and is not intended for commercial use.
## Current Features

### Game Mechanics
- **7-Bag Randomization System**: Ensures fair piece distribution using the modern Tetris standard
- **Hold Queue**: Store a piece for later use (C key)
- **Piece Movement**: Smooth left/right/down movement with collision detection
- **Piece Rotation**: Clockwise and counterclockwise rotation with wall kicks
- **Hard Drop**: Instantly drop pieces to the bottom (Space)
- **Soft Drop**: Speed up piece descent for bonus points (Down arrow)
- **Collision Detection**: Basic detection for board boundaries and piece overlap

### Scoring System
- Line clear scoring (Single: 100, Double: 300, Triple: 500, Tetris: 800)
- Combo bonuses for consecutive line clears
- Back-to-back Tetris bonuses (+1200 points)
- Soft drop and hard drop points

### User Interface
- **Menu Screen**: Welcome screen with game controls
- **Game Screen**: Clean layout showing:
  - Hold queue (left)
  - Game board (center)
  - Next 5 pieces preview (right)
  - Score, level, lines cleared, and combo counter
- **Pause Screen**: Overlay when paused (P key)
- **Game Over Screen**: Display final score with restart option

### Technical Architecture
- **Modular Design**: Separated concerns with dedicated modules
  - `Game`: Core game logic
  - `Board`: 10x20 playfield management
  - `Piece`: Tetromino shapes and rotations
  - `Queue`: 7-bag piece generation
  - `HoldQueue`: Hold functionality
  - `ScoreManager`: Scoring calculations
  - `ScreenManager`: Screen rendering orchestration
  - `screens/`: Individual screen implementations (menu, pause, game over)

## Controls

- **Arrow Keys**: Move piece left/right
- **Up Arrow / X**: Rotate clockwise
- **Z**: Rotate counterclockwise
- **Down Arrow**: Soft drop
- **Space**: Hard drop
- **C**: Hold piece
- **P**: Pause/unpause game

## What's Next?

### Planned Features
- **Gravity System**: Automatic piece descent with level-based speed increases
- **Ghost Pieces**: Preview showing where piece will land
- **Level Progression**: Difficulty increases as lines are cleared
- **Sound Effects**: Audio feedback for actions and line clears
- **Persistent High Scores**: Save and display best scores

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
```

## Project Structure

```
rusttris/
├── src/
│   ├── main.rs                 # Application entry point
│   ├── classes/
│   │   ├── board.rs           # Game board logic
│   │   ├── game.rs            # Core game mechanics
│   │   ├── piece.rs           # Tetromino pieces
│   │   ├── queue.rs           # 7-bag randomization
│   │   ├── hold.rs            # Hold queue
│   │   ├── score_manager.rs   # Scoring system
│   │   ├── screen_manager.rs  # Screen orchestration
│   │   └── screens/           # Individual screens
│   │       ├── menu.rs
│   │       ├── paused.rs
│   │       └── game_over.rs
│   └── enums/
│       └── states.rs          # Game states
├── Cargo.toml
└── README.md
```

## Dependencies

- **eframe** (0.29): Cross-platform GUI framework
- **egui** (0.29): Immediate mode GUI library
- **rand** (0.8): Random number generation for piece shuffling

## License

This project is open source and available for educational purposes.

---

**Status**: Active Development

Current focus: Implementing gravity system and ghost piece preview
