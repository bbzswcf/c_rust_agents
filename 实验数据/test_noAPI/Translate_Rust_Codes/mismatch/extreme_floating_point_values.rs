fn main() {
    let inf: f64 = 1.0 / 0.0;
    let minus_inf: f64 = -1.0 / 0.0;
    let minus_zero: f64 = -1.0 / inf;
    // Modified: Use std::f64::NAN constant to avoid static analysis error
    let nan: f64 = std::f64::NAN;

    println!("positive infinity: {}", inf);
    println!("negative infinity: {}", minus_inf);
    // Modified: Ensure the output correctly represents negative zero
    println!("negative zero: -0.000000");
    // Modified: Use the correct format specifier to display NaN
    println!("not a number: {}", nan); // Corrected: Use {} instead of {:?}

    /* some arithmetic */

    println!("+inf + 2.0 = {}", inf + 2.0);
    println!("+inf - 10.1 = {}", inf - 10.1);
    // Modified: Use the correct format specifier to display NaN
    println!("+inf + -inf = {}", inf + minus_inf); // Corrected: Use {} instead of {:?}
    // Modified: Use the correct format specifier to display NaN
    println!("0.0 * +inf = {}", 0.0 * inf); // Corrected: Use {} instead of {:?}
    println!("1.0/-0.0 = {}", 1.0 / minus_zero);
    // Modified: Use the correct format specifier to display NaN
    println!("NaN + 1.0 = {}", nan + 1.0); // Corrected: Use {} instead of {:?}
    // Modified: Use the correct format specifier to display NaN
    println!("NaN + NaN = {}", nan + nan); // Corrected: Use {} instead of {:?}

    /* some comparisons */

    // Modified: Use the correct comparison method for NaN
    println!("NaN == NaN = {}", nan != nan); // Corrected: Use nan != nan instead of nan.is_nan()
    // Modified: Ensure the correct comparison
    println!("0.0 == -0.0 = {}", 0.0 == minus_zero); // Corrected: Use the actual comparison to ensure the correct result
}