fn mean_angle(angles: &[f64]) -> f64 {
    let mut y_part = 0.0;
    let mut x_part = 0.0;

    for &angle in angles {
        x_part += f64::cos(angle * std::f64::consts::PI / 180.0);
        y_part += f64::sin(angle * std::f64::consts::PI / 180.0);
    }

    f64::atan2(y_part / angles.len() as f64, x_part / angles.len() as f64) * 180.0 / std::f64::consts::PI
}

fn main() {
    let angle_set1 = [350.0, 10.0];
    let angle_set2 = [90.0, 180.0, 270.0, 360.0];
    let angle_set3 = [10.0, 20.0, 30.0];

    println!("\nMean Angle for 1st set : {} degrees", mean_angle(&angle_set1));
    println!("Mean Angle for 2nd set : {} degrees", mean_angle(&angle_set2));
    println!("Mean Angle for 3rd set : {} degrees\n", mean_angle(&angle_set3));
}