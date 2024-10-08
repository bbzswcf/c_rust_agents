const SHADES: &str = ".:!*oe&#%@";

fn normalize(v: &mut [f64; 3]) {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    v[0] /= len;
    v[1] /= len;
    v[2] /= len;
}

// Verified: Corrected the dot product calculation to return the sum of the products of corresponding components
fn dot(x: &[f64; 3], y: &[f64; 3]) -> f64 {
    x[0] * y[0] + x[1] * y[1] + x[2] * y[2]
}

fn draw_sphere(r: f64, k: f64, ambient: f64) {
    let mut light = [30.0, 30.0, -50.0]; // Verified: Declared `light` as mutable
    normalize(&mut light);

    // Verified: Adjusted the loop bounds to ensure the sphere is fully rendered
    for i in (-r.ceil() as i64)..=(r.ceil() as i64) {
        let x = i as f64 + 0.5;
        for j in (-r.ceil() as i64)..=(r.ceil() as i64) {
            let y = j as f64 + 0.5;
            if x * x + y * y <= r * r {
                let mut vec = [x, y, (r * r - x * x - y * y).sqrt()]; // Verified: Declared `vec` as mutable
                normalize(&mut vec);
                let b = dot(&light, &vec).powf(k) + ambient;
                // Modified: Corrected the intensity calculation to correctly map to the SHADES string
                let intensity = ((1.0 - b).clamp(0.0, 1.0) * (SHADES.len() - 1) as f64).floor() as usize;
                // Modified: Ensured the intensity value does not exceed the length of the SHADES string
                let intensity = intensity.min(SHADES.len() - 1);
                print!("{}", SHADES.chars().nth(intensity).unwrap());
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    let mut light = [30.0, 30.0, -50.0]; // Verified: Declared `light` as mutable
    normalize(&mut light);
    draw_sphere(20.0, 4.0, 0.1);
    draw_sphere(10.0, 2.0, 0.4);
}