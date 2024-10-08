use std::f64::consts::PI;

fn mean_angle(angles: &[f64]) -> f64 {
    let mut y_part = 0.0;
    let mut x_part = 0.0;

    for &angle in angles {
        // Convert angle to radians before applying trigonometric functions
        let angle_rad = angle * PI / 180.0;
        x_part += angle_rad.cos();
        y_part += angle_rad.sin();
    }

    // Calculate the mean angle in radians
    let mean_angle_rad = f64::atan2(y_part / angles.len() as f64, x_part / angles.len() as f64);
    
    // Convert the mean angle back to degrees and round to 6 decimal places
    (mean_angle_rad * 180.0 / PI * 1e6).round() / 1e6
}

fn main() {
    let angle_set1 = [350.0, 10.0];
    let angle_set2 = [90.0, 180.0, 270.0, 360.0];
    let angle_set3 = [10.0, 20.0, 30.0];

    // Modified: Format the output to 6 decimal places to match the C output
    println!("Mean Angle for 1st set : {:.6} degrees", mean_angle(&angle_set1));
    println!("Mean Angle for 2nd set : {:.6} degrees", mean_angle(&angle_set2));
    println!("Mean Angle for 3rd set : {:.6} degrees", mean_angle(&angle_set3));
}