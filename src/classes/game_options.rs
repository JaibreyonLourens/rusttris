use crate::enums::game_actions::GameAction;
use egui::ahash::{HashMap, HashMapExt};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// Helper functions to convert egui::Key to/from string
fn key_to_string(key: &egui::Key) -> String {
    format!("{:?}", key)
}

fn string_to_key(s: &str) -> Option<egui::Key> {
    match s {
        "ArrowLeft" => Some(egui::Key::ArrowLeft),
        "ArrowRight" => Some(egui::Key::ArrowRight),
        "ArrowDown" => Some(egui::Key::ArrowDown),
        "ArrowUp" => Some(egui::Key::ArrowUp),
        "Space" => Some(egui::Key::Space),
        "Z" => Some(egui::Key::Z),
        "X" => Some(egui::Key::X),
        "C" => Some(egui::Key::C),
        "R" => Some(egui::Key::R),
        "P" => Some(egui::Key::P),
        "Escape" => Some(egui::Key::Escape),
        _ => None,
    }
}

// Serializable version of key bindings
#[derive(Serialize, Deserialize)]
struct SerializableBindings {
    bindings: Vec<(GameAction, String)>,
}

#[derive(Debug, Clone)]
pub struct GameOptions {
    pub key_bindings: HashMap<GameAction, egui::Key>,
    pub ghost_piece_alpha: u8,
    pub das_delay: f32,
    pub arr_delay: f32,
}

impl Default for GameOptions {
    fn default() -> Self {
        let mut key_bindings = HashMap::new();
        key_bindings.insert(GameAction::MoveLeft, egui::Key::ArrowLeft);
        key_bindings.insert(GameAction::MoveRight, egui::Key::ArrowRight);
        key_bindings.insert(GameAction::SoftDrop, egui::Key::ArrowDown);
        key_bindings.insert(GameAction::HardDrop, egui::Key::Space);
        key_bindings.insert(GameAction::RotateCW, egui::Key::Z);
        key_bindings.insert(GameAction::RotateCCW, egui::Key::X);
        key_bindings.insert(GameAction::HoldPiece, egui::Key::C);
        key_bindings.insert(GameAction::RestartGame, egui::Key::R);
        key_bindings.insert(GameAction::PauseGame, egui::Key::P);
        key_bindings.insert(GameAction::ResumeGame, egui::Key::Escape);
        GameOptions {
            key_bindings,
            ghost_piece_alpha: 100,
            das_delay: 150.0,
            arr_delay: 50.0,
        }
    }
}

impl Serialize for GameOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;

        let bindings: Vec<(GameAction, String)> = self
            .key_bindings
            .iter()
            .map(|(action, key)| (*action, key_to_string(key)))
            .collect();

        let mut state = serializer.serialize_struct("GameOptions", 4)?;
        state.serialize_field("key_bindings", &bindings)?;
        state.serialize_field("ghost_piece_alpha", &self.ghost_piece_alpha)?;
        state.serialize_field("das_delay", &self.das_delay)?;
        state.serialize_field("arr_delay", &self.arr_delay)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for GameOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            key_bindings: Vec<(GameAction, String)>,
            ghost_piece_alpha: u8,
            das_delay: f32,
            arr_delay: f32,
        }

        let helper = Helper::deserialize(deserializer)?;

        let mut key_bindings = HashMap::default();
        for (action, key_str) in helper.key_bindings {
            if let Some(key) = string_to_key(&key_str) {
                key_bindings.insert(action, key);
            }
        }

        Ok(GameOptions {
            key_bindings,
            ghost_piece_alpha: helper.ghost_piece_alpha,
            das_delay: helper.das_delay,
            arr_delay: helper.arr_delay,
        })
    }
}

impl GameOptions {
    pub fn save(&self) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write("settings.json", json)?;
        Ok(())
    }

    pub fn load() -> Self {
        std::fs::read_to_string("settings.json")
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }
}