fn price_fix(x: f64) -> f64 {
    let table: [[f64; 2]; 21] = [
        [0.06, 0.10], [0.11, 0.18], [0.16, 0.26], [0.21, 0.32],
        [0.26, 0.38], [0.31, 0.44], [0.36, 0.50], [0.41, 0.54],
        [0.46, 0.58], [0.51, 0.62], [0.56, 0.66], [0.61, 0.70],
        [0.66, 0.74], [0.71, 0.78], [0.76, 0.82], [0.81, 0.86],
        [0.86, 0.90], [0.91, 0.94], [0.96, 0.98], [1.01, 1.00],
        [-1.0, 0.0], // guarding element
    ];

    for entry in table.iter() {
        if entry[0] < 0.0 {
            break;
        }
        if x < entry[0] {
            return entry[1];
        }
    }

    std::process::abort(); // what else to do?
}

fn main() {
    for i in 0..=100 {
        let x = i as f64 / 100.0;
        println!("{:.2} {:.2}", x, price_fix(x));
    }
}