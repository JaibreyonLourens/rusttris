#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver,
}