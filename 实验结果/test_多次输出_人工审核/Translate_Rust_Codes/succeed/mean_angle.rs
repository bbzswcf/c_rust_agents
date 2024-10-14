fn mean_angle(angles: &[f64]) -> f64 {
    let (x_part, y_part): (f64, f64) = angles
        .iter()
        .map(|&angle| (angle.to_radians(), angle.to_radians()))
        .map(|(cos_val, sin_val)| (cos_val.cos(), sin_val.sin()))
        .fold((0.0, 0.0), |(x_acc, y_acc), (cos_val, sin_val)| {
            (x_acc + cos_val, y_acc + sin_val)
        });

    y_part.atan2(x_part).to_degrees()
}

fn main() {
    let angle_set1 = [350.0, 10.0];
    let angle_set2 = [90.0, 180.0, 270.0, 360.0];
    let angle_set3 = [10.0, 20.0, 30.0];

    println!("Mean Angle for 1st set: {:.6} degrees", mean_angle(&angle_set1));
    println!("Mean Angle for 2nd set: {:.6} degrees", mean_angle(&angle_set2));
    println!("Mean Angle for 3rd set: {:.6} degrees", mean_angle(&angle_set3));
}