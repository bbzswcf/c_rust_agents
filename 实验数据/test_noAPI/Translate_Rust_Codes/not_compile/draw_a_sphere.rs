// Added: Ensure the `lazy_static` crate is added to `Cargo.toml` under `[dependencies]`
// [dependencies]
// lazy_static = "1.4.0"

use lazy_static::lazy_static; // Added: Use lazy_static to initialize the variable in a safe manner
// Removed: Unused import `std::sync::Mutex`

const SHADES: &str = ".:!*oe&#%@";

lazy_static! {
    static ref LIGHT: [f64; 3] = {
        let mut light = [30.0, 30.0, -50.0];
        normalize(&mut light);
        light
    }; // Modified: Use lazy_static for safe initialization and access
}

fn normalize(v: &mut [f64; 3]) {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    v[0] /= len;
    v[1] /= len;
    v[2] /= len;
}

fn dot(x: &[f64; 3], y: &[f64; 3]) -> f64 { // Added: Define the dot function before it is used
    let d = x[0] * y[0] + x[1] * y[1] + x[2] * y[2];
    if d < 0.0 { -d } else { 0.0 }
}

fn draw_sphere(r: f64, k: f64, ambient: f64) {
    let r_squared = r * r;
    let r_double = 2.0 * r;
    let shades_len = SHADES.len();

    for i in (r.floor() as i64)..=(r.ceil() as i64) {
        let x = i as f64 + 0.5;
        for j in (r_double.floor() as i64)..=(r_double.ceil() as i64) {
            let y = j as f64 / 2.0 + 0.5;
            if x * x + y * y <= r_squared {
                let mut vec = [x, y, (r_squared - x * x - y * y).sqrt()];
                normalize(&mut vec);
                // Modified: Dereference LIGHT to pass a reference to the actual array
                let b = dot(&*LIGHT, &vec).powf(k) + ambient;
                let intensity = ((1.0 - b) * (shades_len - 1) as f64) as usize;
                let intensity = intensity.clamp(0, shades_len - 2);
                print!("{}", SHADES.chars().nth(intensity).unwrap());
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    draw_sphere(20.0, 4.0, 0.1);
    draw_sphere(10.0, 2.0, 0.4);
}