use std::f64::consts::PI;

const SHADES: &str = ".:!*oe&#%@";

fn normalize(v: &mut [f64; 3]) {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    // Ensure that the length `len` is not zero before performing the division
    if len > 1e-6 { // Modified: Use a more appropriate epsilon value for floating-point comparisons
        v[0] /= len;
        v[1] /= len;
        v[2] /= len;
    }
}

fn dot(x: &[f64; 3], y: &[f64; 3]) -> f64 {
    let d = x[0] * y[0] + x[1] * y[1] + x[2] * y[2];
    // Return the absolute value of `d` to ensure non-negative intensity calculation
    d.abs()
}

fn draw_sphere(r: f64, k: f64, ambient: f64, light: &[f64; 3]) {
    let mut vec = [0.0; 3];
    // Adjust the loop bounds to ensure the correct range for drawing the sphere
    for i in (-r as i64)..=(r as i64) {
        let x = i as f64 + 0.5;
        for j in (-r as i64)..=(r as i64) {
            // Correctly calculate the `y` coordinate based on the sphere's radius and loop index
            let y = j as f64 + 0.5;
            if x * x + y * y <= r * r {
                vec[0] = x;
                vec[1] = y;
                vec[2] = (r * r - x * x - y * y).sqrt();
                normalize(&mut vec);
                let b = dot(light, &vec).powf(k) + ambient;
                // Correctly map the intensity to the range of SHADES
                let intensity = ((b * (SHADES.len() - 1) as f64).round() as usize).clamp(0, SHADES.len() - 1);
                // Handle potential out-of-bounds cases gracefully
                print!("{}", SHADES.chars().nth(intensity).unwrap_or(' '));
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    // Normalize the light vector before using it in the draw_sphere function
    let mut light = [0.4242640687119285, 0.4242640687119285, -0.7071067811865475];
    normalize(&mut light);

    // Pass the normalized light vector to the draw_sphere function
    draw_sphere(20.0, 4.0, 0.1, &light);
    draw_sphere(10.0, 2.0, 0.4, &light);
}