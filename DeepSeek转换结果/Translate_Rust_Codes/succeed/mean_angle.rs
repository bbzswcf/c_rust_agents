use std::f64::consts::PI;

fn mean_angle(angles: &[f64]) -> f64 {
    let size = angles.len() as f64;
    let (x_part, y_part) = angles.iter().fold((0.0, 0.0), |(x, y), &angle| {
        (x + (angle * PI / 180.0).cos(), y + (angle * PI / 180.0).sin())
    });
    let mean_angle = f64::atan2(y_part / size, x_part / size) * 180.0 / PI;
    mean_angle
}

fn main() {
    let angle_set1 = [350.0, 10.0];
    let angle_set2 = [90.0, 180.0, 270.0, 360.0];
    let angle_set3 = [10.0, 20.0, 30.0];

    println!("\nMean Angle for 1st set : {:.6} degrees", mean_angle(&angle_set1));
    println!("Mean Angle for 2nd set : {:.6} degrees", mean_angle(&angle_set2));
    println!("Mean Angle for 3rd set : {:.6} degrees\n", mean_angle(&angle_set3));
}