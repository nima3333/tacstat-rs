// use crate::utils::computation::haversine_distance;
use crate::utils::computation::haversine_distance_with_altitude;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub lat: f32,
    pub long: f32,
    pub alt: f32,
}

impl Position {
    pub fn new(lat: f32, long: f32, alt: f32) -> Self {
        Self { lat, long, alt }
    }

    pub fn update(&mut self, lat: f32, long: f32, alt: f32) {
        // if let Some(lat) = lat {
            self.lat = lat;
        // }
        // if let Some(long) = long {
            self.long = long;
        // }
        // if let Some(alt) = alt {
            self.alt = alt;
        // }
    }

    pub fn distance_to(&self, other: &Position) -> f32 {
        // haversine_distance(self.lat, self.long, other.lat, other.long)
        haversine_distance_with_altitude(self.lat, self.long, self.alt, other.lat, other.long, other.alt)
    }
}