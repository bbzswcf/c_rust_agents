const SHADES: &str = ".:!*oe&#%@";

fn normalize(v: &mut [f64; 3]) {
    let len = (v[0].powi(2) + v[1].powi(2) + v[2].powi(2)).sqrt();
    v[0] /= len;
    v[1] /= len;
    v[2] /= len;
}

fn dot(x: &[f64; 3], y: &[f64; 3]) -> f64 {
    let d = x[0] * y[0] + x[1] * y[1] + x[2] * y[2];
    if d < 0.0 { -d } else { 0.0 }
}

fn draw_sphere(r: f64, k: f64, ambient: f64, light: &[f64; 3]) {
    let r_squared = r * r;
    let r_double = 2.0 * r;

    for i in (-r.floor() as i32)..=(r.ceil() as i32) {
        let x = i as f64 + 0.5;
        for j in (-r_double.floor() as i32)..=(r_double.ceil() as i32) {
            let y = j as f64 / 2.0 + 0.5;
            if x * x + y * y <= r_squared {
                let mut vec = [x, y, (r_squared - x * x - y * y).sqrt()];
                normalize(&mut vec);
                let b = dot(light, &vec).powf(k) + ambient;
                let intensity = ((1.0 - b) * (SHADES.len() - 1) as f64).round() as usize;
                let intensity = intensity.clamp(0, SHADES.len() - 2);
                print!("{}", SHADES.chars().nth(intensity).unwrap());
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    let mut light = [30.0, 30.0, -50.0];
    normalize(&mut light);
    draw_sphere(20.0, 4.0, 0.1, &light);
    draw_sphere(10.0, 2.0, 0.4, &light);
}