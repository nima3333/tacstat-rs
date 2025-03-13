#[derive(Debug, Clone)]
pub struct PlayerInfo {
    pub name: String,
    pub vehicle: String,
    pub creation_time: f64,
    pub deletion_time: f64,
    pub time_in_game: f64
}

impl PlayerInfo {
    pub fn new(name: String, vehicle: String, creation_time: f64) -> Self {
        Self {
            name,
            vehicle,
            creation_time,
            deletion_time: -1.0,
            time_in_game: -1.0,
        }
    }

    pub fn mark_deleted(&mut self, time: f64) {
        self.deletion_time = time;
    }
}


#[derive(Debug, Clone)]
pub struct PartialPlayerInfo {
    pub name: String,
    pub vehicle: String,
}


impl PartialPlayerInfo {
    pub fn new(name: String, vehicle: String) -> Self {
        Self {
            name,
            vehicle,
        }
    }
}

