fn main() {
    let inf = 1.0 / 0.0;
    let minus_inf = -1.0 / 0.0;
    let minus_zero = -1.0 / inf;
    // Modified: Use f64::NAN constant instead of 0.0 / 0.0
    let nan = f64::NAN;

    println!("positive infinity: {}", inf);
    println!("negative infinity: {}", minus_inf);
    println!("negative zero: {}", minus_zero);
    println!("not a number: {}", nan);

    /* some arithmetic */

    println!("+inf + 2.0 = {}", inf + 2.0);
    println!("+inf - 10.1 = {}", inf - 10.1);
    println!("+inf + -inf = {}", inf + minus_inf);
    println!("0.0 * +inf = {}", 0.0 * inf);
    println!("1.0/-0.0 = {}", 1.0 / minus_zero);
    println!("NaN + 1.0 = {}", nan + 1.0);
    println!("NaN + NaN = {}", nan + nan);

    /* some comparisons */

    // Modified: Use is_nan method to check for NaN
    println!("NaN == NaN = {}", if nan.is_nan() { "true" } else { "false" });
    println!("0.0 == -0.0 = {}", if 0.0 == minus_zero { "true" } else { "false" });
}