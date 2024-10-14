const R: f64 = 6371.0;
// Modified: Use the constant `f64::consts::PI` directly instead of approximating the value of PI
const TO_RAD: f64 = std::f64::consts::PI / 180.0;

fn dist(th1: f64, ph1: f64, th2: f64, ph2: f64) -> f64 {
    let mut ph1 = ph1 - ph2;
    ph1 *= TO_RAD;
    let th1 = th1 * TO_RAD;
    let th2 = th2 * TO_RAD;

    // Modified: Use the constants from `std::f64::consts` module
    let dz = th1.sin() - th2.sin();
    let dx = ph1.cos() * th1.cos() - th2.cos();
    let dy = ph1.sin() * th1.cos();

    // Modified: Correct the calculation for the angular distance using the Haversine formula
    let a = (dx * dx + dy * dy + dz * dz) / 4.0;
    let angle = 2.0 * a.sqrt().asin();
    angle * R
}

fn main() {
    // Modified: Ensure the number of arguments passed to the `dist` function matches the number of arguments specified in its signature
    let d = dist(36.12, -86.67, 33.94, -118.40);
    println!("dist: {:.1} km ({:.1} mi.)", d, d / 1.609344);
}