const R: f64 = 6371.0; // Earth's radius in kilometers
const TO_RAD: f64 = std::f64::consts::PI / 180.0; // Conversion factor from degrees to radians

fn dist(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    // Convert latitude and longitude from degrees to radians
    let lat1 = lat1 * TO_RAD;
    let lon1 = lon1 * TO_RAD;
    let lat2 = lat2 * TO_RAD;
    let lon2 = lon2 * TO_RAD;

    // Calculate differences in latitude and longitude
    let dlat = lat2 - lat1;
    let dlon = lon2 - lon1;

    // Apply the Haversine formula
    let a = (dlat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    let d = R * c;

    d
}

fn main() {
    let d = dist(36.12, -86.67, 33.94, -118.4);
    println!("dist: {:.1} km ({:.1} mi.)", d, d / 1.609344);
}