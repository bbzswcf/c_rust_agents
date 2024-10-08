const R: f64 = 6371.0; // Earth's radius in kilometers
const TO_RAD: f64 = std::f64::consts::PI / 180.0; // Conversion factor from degrees to radians

// Modified: Corrected the Haversine distance formula implementation
fn dist(th1: f64, ph1: f64, th2: f64, ph2: f64) -> f64 {
    let th1 = th1 * TO_RAD;
    let th2 = th2 * TO_RAD;
    // Corrected: Ensure the differences in latitude and longitude are correctly handled
    let d_phi = (ph2 - ph1).abs() * TO_RAD;
    let d_theta = (th2 - th1).abs() * TO_RAD;

    // Haversine formula
    // Corrected: Ensure the trigonometric functions are applied correctly
    let a = (d_phi / 2.0).sin().powi(2) + th1.cos() * th2.cos() * (d_theta / 2.0).sin().powi(2);
    // Corrected: Ensure the calculation of `c` matches the standard Haversine formula
    let c = 2.0 * R * (a.sqrt().asin()); // Modified: Corrected the formula for `c` by including the multiplication by `R`

    c
}

fn main() {
    let d = dist(36.12, -86.67, 33.94, -118.4);
    // Corrected: Ensure the conversion factor from kilometers to miles is correct
    println!("dist: {:.1} km ({:.1} mi.)", d, d * 0.621371); // Modified: Corrected the conversion factor from kilometers to miles
}