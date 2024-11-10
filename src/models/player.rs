#[derive(Debug, Clone)]
pub struct PlayerInfo {
    pub name: String,
    pub vehicle: String,
    pub creation_time: f64,
    pub deletion_time: f64,
}

impl PlayerInfo {
    pub fn new(name: String, vehicle: String, creation_time: f64) -> Self {
        Self {
            name,
            vehicle,
            creation_time,
            deletion_time: 0.0,
        }
    }

    pub fn mark_deleted(&mut self, time: f64) {
        self.deletion_time = time;
    }
}

