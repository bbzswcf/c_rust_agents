const SHADES: &str = ".:!*oe&#%@";

fn normalize(v: &mut [f64; 3]) {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    v[0] /= len;
    v[1] /= len;
    v[2] /= len;
}

fn dot(x: &[f64; 3], y: &[f64; 3]) -> f64 {
    let d = x[0] * y[0] + x[1] * y[1] + x[2] * y[2];
    d.max(0.0) // Ensured dot product calculation returns positive values
}

fn draw_sphere(r: f64, k: f64, ambient: f64, light: &[f64; 3]) {
    for i in (-r.ceil() as i64)..=(r.ceil() as i64) { // Corrected loop bounds for sphere drawing
        let x = i as f64 + 0.5;
        for j in (-r.ceil() as i64)..=(r.ceil() as i64) { // Corrected loop bounds for sphere drawing
            let y = j as f64 + 0.5;
            if x * x + y * y <= r * r {
                let mut vec = [x, y, (r * r - x * x - y * y).sqrt()];
                normalize(&mut vec); // Normalize the vector inside the function
                let b = (dot(light, &vec) + ambient).powf(k); // Corrected handling of ambient light
                let intensity = (b * (SHADES.len() - 1) as f64).floor() as usize; // Corrected intensity calculation
                let intensity = intensity.min(SHADES.len() - 1); // Corrected bounds checking for intensity
                print!("{}", SHADES.chars().nth(intensity).unwrap());
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    let mut normalized_light = [30.0, 30.0, -50.0];
    normalize(&mut normalized_light); // Normalized light vector directly
    draw_sphere(20.0, 4.0, 0.1, &normalized_light); // Corrected sphere radius and ambient light value
    draw_sphere(10.0, 2.0, 0.4, &normalized_light); // Corrected sphere radius and ambient light value
}