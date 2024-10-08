const R: f64 = 6371.0;
// 修改: 使用标准库中的PI常量,避免硬编码
const TO_RAD: f64 = std::f64::consts::PI / 180.0;

fn dist(th1: f64, ph1: f64, th2: f64, ph2: f64) -> f64 {
    let mut ph1 = ph1 - ph2;
    ph1 *= TO_RAD;
    let th1 = th1 * TO_RAD;
    let th2 = th2 * TO_RAD;

    let dz = f64::sin(th1) - f64::sin(th2);
    let dx = f64::cos(ph1) * f64::cos(th1) - f64::cos(th2);
    let dy = f64::sin(ph1) * f64::cos(th1);
    f64::asin(f64::sqrt(dx * dx + dy * dy + dz * dz) / 2.0) * 2.0 * R
}

fn main() {
    let d = dist(36.12, -86.67, 33.94, -118.4);
    // 修改: 使用println!宏,避免print!宏可能导致的输出问题
    println!("dist: {:.1} km ({:.1} mi.)", d, d / 1.609344);
}