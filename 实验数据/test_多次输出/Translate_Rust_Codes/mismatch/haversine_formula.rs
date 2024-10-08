const R: f64 = 6371.0;
const TO_RAD: f64 = std::f64::consts::PI / 180.0;

fn dist(th1: f64, ph1: f64, th2: f64, ph2: f64) -> f64 {
    let ph1 = ph1 - ph2;
    let th1 = th1 * TO_RAD;
    let th2 = th2 * TO_RAD;
    let ph1 = ph1 * TO_RAD;

    let dz = th1.sin() - th2.sin();
    let dx = ph1.cos() * th1.cos() - th2.cos();
    let dy = ph1.sin() * th1.cos();

    (dx * dx + dy * dy + dz * dz).sqrt().asin() * 2.0 * R
}

fn main() {
    let d = dist(36.12, -86.67, 33.94, -118.4);
    println!("dist: {:.1} km ({:.1} mi.)", d, d / 1.609344);
}