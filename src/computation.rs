use std::f32::consts::PI;

// Earth's radius in kilometers
const EARTH_RADIUS_KM: f32 = 6371.0;

/// Converts degrees to radians.
fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

/// Calculates the Haversine distance between two latitude/longitude points.
pub fn haversine_distance(lat1: f32, lon1: f32, lat2: f32, lon2: f32) -> f32 {
    let lat1_rad = degrees_to_radians(lat1);
    let lon1_rad = degrees_to_radians(lon1);
    let lat2_rad = degrees_to_radians(lat2);
    let lon2_rad = degrees_to_radians(lon2);

    let dlat = lat2_rad - lat1_rad;
    let dlon = lon2_rad - lon1_rad;

    let a = (dlat / 2.0).sin().powi(2)
          + lat1_rad.cos() * lat2_rad.cos() * (dlon / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    EARTH_RADIUS_KM * c
}
