use std::io::{self, Write};
use std::cmp::{min, max};

const SHADES: &str = ".:!*oe&#%@";

fn normalize(v: &mut [f64; 3]) {
    let len = f64::sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);
    v[0] /= len;
    v[1] /= len;
    v[2] /= len;
}

fn dot(x: &[f64; 3], y: &[f64; 3]) -> f64 {
    let d = x[0] * y[0] + x[1] * y[1] + x[2] * y[2];
    f64::abs(d)
}

fn draw_sphere(r: f64, k: f64, ambient: f64, light: &[f64; 3]) {
    let mut vec = [0.0; 3];
    for i in (f64::floor(-r) as i64)..=(f64::ceil(r) as i64) {
        let x = i as f64 + 0.5;
        for j in (f64::floor(-2.0 * r) as i64)..=(f64::ceil(2.0 * r) as i64) {
            let y = j as f64 / 2.0 + 0.5;
            if x * x + y * y <= r * r {
                vec[0] = x;
                vec[1] = y;
                vec[2] = f64::sqrt(r * r - x * x - y * y);
                normalize(&mut vec);
                let b = f64::powf(dot(light, &vec), k) + ambient;
                let intensity = (1.0 - b) * (SHADES.len() - 1) as f64;
                let intensity = f64::max(f64::min(intensity, (SHADES.len() - 1) as f64), 0.0) as usize;
                io::stdout().write_all(&[SHADES.as_bytes()[intensity]]).unwrap();
            } else {
                io::stdout().write_all(&[b' ']).unwrap();
            }
        }
        io::stdout().write_all(&[b'\n']).unwrap();
    }
}

fn main() {
    let mut light = [30.0, 30.0, -50.0];
    normalize(&mut light);
    draw_sphere(20.0, 4.0, 0.1, &light);
    draw_sphere(10.0, 2.0, 0.4, &light);
}