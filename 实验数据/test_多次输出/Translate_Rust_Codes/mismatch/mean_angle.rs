fn mean_angle(angles: &[f64]) -> f64 {
    let (x_part, y_part) = angles.iter().fold((0.0, 0.0), |(x, y), &angle| {
        (x + (angle * std::f64::consts::PI / 180.0).cos(), y + (angle * std::f64::consts::PI / 180.0).sin())
    });

    (y_part / angles.len() as f64).atan2(x_part / angles.len() as f64) * 180.0 / std::f64::consts::PI
}

fn main() {
    let angle_set1 = [350.0, 10.0];
    let angle_set2 = [90.0, 180.0, 270.0, 360.0];
    let angle_set3 = [10.0, 20.0, 30.0];

    println!("Mean Angle for 1st set: {} degrees", mean_angle(&angle_set1));
    println!("Mean Angle for 2nd set: {} degrees", mean_angle(&angle_set2));
    println!("Mean Angle for 3rd set: {} degrees", mean_angle(&angle_set3));
}