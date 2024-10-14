fn main() {
    let inf: f64 = 1.0 / 0.0;
    let minus_inf: f64 = -1.0 / 0.0;
    // Modified: Use -0.0 to correctly produce negative zero
    let minus_zero: f64 = -0.0;
    // Modified: Use std::f64::NAN constant to ensure correct representation
    let nan: f64 = std::f64::NAN;

    println!("positive infinity: {}", inf);
    println!("negative infinity: {}", minus_inf);
    // Modified: Use the correct format specifier to display negative zero without scientific notation
    println!("negative zero: {:.6}", minus_zero);
    // Modified: Use the correct format specifier to display NaN without scientific notation
    println!("not a number: {}", nan); // Changed from {:.6} to {}

    println!("+inf + 2.0 = {}", inf + 2.0);
    println!("+inf - 10.1 = {}", inf - 10.1);
    // Modified: Use the correct format specifier to display NaN without scientific notation
    println!("+inf + -inf = {}", inf + minus_inf); // Changed from {:.6} to {}
    // Modified: Use the correct format specifier to display NaN without scientific notation
    println!("0.0 * +inf = {}", 0.0 * inf); // Changed from {:.6} to {}
    println!("1.0/-0.0 = {}", 1.0 / minus_zero);
    // Modified: Use the correct format specifier to display NaN without scientific notation
    println!("NaN + 1.0 = {}", nan + 1.0); // Changed from {:.6} to {}
    // Modified: Use the correct format specifier to display NaN without scientific notation
    println!("NaN + NaN = {}", nan + nan); // Changed from {:.6} to {}

    println!("NaN == NaN = {}", if nan.is_nan() { "false" } else { "true" });
    println!("0.0 == -0.0 = {}", if 0.0 == minus_zero { "true" } else { "false" });
}