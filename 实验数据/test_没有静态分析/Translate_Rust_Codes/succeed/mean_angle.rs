use std::f64::consts::PI;

fn mean_angle(angles: &[f64]) -> f64 {
    let mut y_part = 0.0;
    let mut x_part = 0.0;

    for &angle in angles {
        x_part += (angle * PI / 180.0).cos();
        y_part += (angle * PI / 180.0).sin();
    }

    f64::atan2(y_part / angles.len() as f64, x_part / angles.len() as f64) * 180.0 / PI
}

fn main() {
    let angle_set1 = [350.0, 10.0];
    let angle_set2 = [90.0, 180.0, 270.0, 360.0];
    let angle_set3 = [10.0, 20.0, 30.0];

    // Modified: Rounded the result to 6 decimal places to match the C output more closely
    println!("Mean Angle for 1st set : {:.6} degrees", mean_angle(&angle_set1));
    // Modified: Applied the same formatting to the 2nd set mean angle as used for the 1st and 3rd sets
    println!("Mean Angle for 2nd set : {:.6} degrees", mean_angle(&angle_set2));
    // Modified: Rounded the result to 6 decimal places to match the C output more closely
    println!("Mean Angle for 3rd set : {:.6} degrees", mean_angle(&angle_set3));
}