#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    PlayerCreation,
    PlayerSelection,
    Menu,
    Playing,
    Paused,
    GameOver,
}