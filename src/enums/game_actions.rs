#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum GameAction {
    MoveLeft,
    MoveRight,
    SoftDrop,
    HardDrop,
    RotateCW,
    RotateCCW,
    HoldPiece,
    RestartGame,
    PauseGame,
    ResumeGame,
}