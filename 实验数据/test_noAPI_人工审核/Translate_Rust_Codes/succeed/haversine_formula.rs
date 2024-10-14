const R: f64 = 6371.0;
const TO_RAD: f64 = std::f64::consts::PI / 180.0;

fn dist(th1: f64, ph1: f64, th2: f64, ph2: f64) -> f64 {
    // Modified: Correctly assign ph1 as the difference in longitudes and convert latitudes to radians
    let (ph1, th1, th2) = ((ph1 - ph2) * TO_RAD, th1 * TO_RAD, th2 * TO_RAD);

    // Modified: Use the Haversine formula to calculate the great-circle distance
    let sin_half_d_lat = ((th2 - th1) / 2.0).sin();
    let sin_half_d_lon = (ph1 / 2.0).sin();

    let a = sin_half_d_lat * sin_half_d_lat + th1.cos() * th2.cos() * sin_half_d_lon * sin_half_d_lon;
    let c = 2.0 * a.sqrt().asin();

    R * c
}

fn main() {
    let d = dist(36.12, -86.67, 33.94, -118.4);
    println!("dist: {:.1} km ({:.1} mi.)", d, d / 1.609344);
}