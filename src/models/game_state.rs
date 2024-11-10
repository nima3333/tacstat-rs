use std::collections::HashMap;
use super::{PlayerInfo, Position};

#[derive(Debug, Default)]
pub struct GameState {
    pub current_time: f64,
    pub players: HashMap<i32, PlayerInfo>,
    pub positions: HashMap<i32, Position>,
    pub weapon_stats: HashMap<i32, HashMap<String, i32>>,
}

impl GameState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_player(&mut self, id: i32, name: String, vehicle: String, pos: Position) {
        self.players.insert(
            id,
            PlayerInfo::new(name, vehicle, self.current_time)
        );
        self.positions.insert(id, pos);
    }

    // ... rest of the implementation
}

