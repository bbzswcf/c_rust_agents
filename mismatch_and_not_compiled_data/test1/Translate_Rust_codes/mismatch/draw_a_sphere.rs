const SHADES: &str = ".:!*oe&#%@";

fn normalize(v: &mut [f64; 3]) {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len < 1e-6 { // Ensure that the epsilon value is appropriate for floating-point comparisons
        return;
    }
    v[0] /= len;
    v[1] /= len;
    v[2] /= len;
}

fn dot(x: &[f64; 3], y: &[f64; 3]) -> f64 {
    let d = x[0] * y[0] + x[1] * y[1] + x[2] * y[2]; // Ensure the vectors `x` and `y` are correctly normalized before performing the dot product
    d
}

fn draw_sphere(r: f64, k: f64, ambient: f64, light: &[f64; 3]) {
    for i in (r.floor() as i64)..=(r.ceil() as i64) {
        let x = i as f64 + 0.5;
        for j in ((2.0 * r).floor() as i64)..=((2.0 * r).ceil() as i64) {
            let y = j as f64 / 2.0 + 0.5;
            if x * x + y * y <= r * r {
                let mut vec = [x, y, (r * r - x * x - y * y).sqrt()];
                normalize(&mut vec);
                let b = dot(light, &vec).powf(k) + ambient;
                let b = b.clamp(0.0, 1.0); // Ensure `b` is clamped between 0.0 and 1.0
                let intensity = (b * (SHADES.len() - 1) as f64).round() as usize; // Directly map `b` to the range of `SHADES`
                print!("{}", SHADES.chars().nth(intensity).unwrap_or(' ')); // Use `unwrap_or` to handle out-of-bounds gracefully
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    let light = [30.0, 30.0, -50.0];
    let mut normalized_light = light;
    normalize(&mut normalized_light); // Normalize the light vector only once
    draw_sphere(20.0, 4.0, 0.1, &normalized_light);
    draw_sphere(10.0, 2.0, 0.4, &normalized_light);
}