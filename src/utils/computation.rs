use std::f32::consts::PI;

// Earth's radius in kilometers
const EARTH_RADIUS_M: f32 = 6371000.0;

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

    EARTH_RADIUS_M * c
}

pub fn haversine_distance_with_altitude(
    lat1: f32, lon1: f32, alt1: f32,
    lat2: f32, lon2: f32, alt2: f32
) -> f32 {
    let lat1_rad = degrees_to_radians(lat1);
    let lon1_rad = degrees_to_radians(lon1);
    let lat2_rad = degrees_to_radians(lat2);
    let lon2_rad = degrees_to_radians(lon2);

    let dlat = lat2_rad - lat1_rad;
    let dlon = lon2_rad - lon1_rad;

    // Haversine formula to compute the central angle between the two points.
    let a = (dlat / 2.0).sin().powi(2)
          + lat1_rad.cos() * lat2_rad.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    // Effective radii for each point (Earth radius + altitude)
    let r1 = EARTH_RADIUS_M + alt1;
    let r2 = EARTH_RADIUS_M + alt2;

    // Compute the Euclidean distance using the spherical law of cosines.
    (r1 * r1 + r2 * r2 - 2.0 * r1 * r2 * c.cos()).sqrt()
}