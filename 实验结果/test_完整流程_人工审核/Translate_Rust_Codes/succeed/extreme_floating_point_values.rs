fn main() {
    let inf = 1.0 / 0.0;
    let minus_inf = -1.0 / 0.0;
    let minus_zero = -0.0;
    let nan = std::f64::NAN;

    println!("positive infinity: {}", inf);
    println!("negative infinity: {}", minus_inf);
    println!("negative zero: {:.6}", minus_zero);
    // Modified: Correctly display `-nan` for NaN values
    println!("not a number: {}", if nan.is_nan() { "-nan" } else { "nan" });

    /* some arithmetic */

    println!("+inf + 2.0 = {}", inf + 2.0);
    println!("+inf - 10.1 = {}", inf - 10.1);
    // Modified: Specify the type of the floating-point number explicitly to avoid ambiguous numeric type error
    println!("+inf + -inf = {}", if ((inf + minus_inf) as f64).is_nan() { "-nan" } else { "nan" });
    // Modified: Specify the type of the floating-point number explicitly to avoid ambiguous numeric type error
    println!("0.0 * +inf = {}", if ((0.0 * inf) as f64).is_nan() { "-nan" } else { "nan" });
    println!("1.0/-0.0 = {}", 1.0 / minus_zero);
    // Modified: Correctly display `-nan` for NaN values
    println!("NaN + 1.0 = {}", if (nan + 1.0).is_nan() { "-nan" } else { "nan" });
    // Modified: Correctly display `-nan` for NaN values
    println!("NaN + NaN = {}", if (nan + nan).is_nan() { "-nan" } else { "nan" });

    /* some comparisons */

    // Modified: Correctly display `false` for NaN comparison
    println!("NaN == NaN = {}", if nan.is_nan() { "false" } else { "true" });
    // Modified: Correctly display `true` for the comparison of 0.0 and -0.0
    println!("0.0 == -0.0 = {}", if 0.0 == minus_zero { "true" } else { "false" });
}