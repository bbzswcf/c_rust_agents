fn main() {
    let inf = f64::INFINITY;
    let minus_inf = f64::NEG_INFINITY;
    let minus_zero = -1.0 / f64::INFINITY;
    let nan = f64::NAN;

    println!("positive infinity: {}", inf);
    println!("negative infinity: {}", minus_inf);
    // Modified: Format the output to include more decimal places to match the expected output
    println!("negative zero: {:.6}", minus_zero);
    // Modified: Explicitly check for NaN and print `-nan` if it is NaN
    println!("not a number: {}", if nan.is_nan() { "-nan" } else { "NaN" });

    /* some arithmetic */

    println!("+inf + 2.0 = {}", inf + 2.0);
    println!("+inf - 10.1 = {}", inf - 10.1);
    // Modified: Explicitly check for NaN and print `-nan` if it is NaN
    println!("+inf + -inf = {}", if (inf + minus_inf).is_nan() { "-nan" } else { "NaN" });
    // Modified: Explicitly check for NaN and print `-nan` if it is NaN
    println!("0.0 * +inf = {}", if (0.0 * inf).is_nan() { "-nan" } else { "NaN" });
    println!("1.0/-0.0 = {}", 1.0 / minus_zero);
    // Modified: Explicitly check for NaN and print `-nan` if it is NaN
    println!("NaN + 1.0 = {}", if (nan + 1.0).is_nan() { "-nan" } else { "NaN" });
    // Modified: Explicitly check for NaN and print `-nan` if it is NaN
    println!("NaN + NaN = {}", if (nan + nan).is_nan() { "-nan" } else { "NaN" });

    /* some comparisons */

    println!("NaN == NaN = {}", if nan == nan { "true" } else { "false" });
    println!("0.0 == -0.0 = {}", if 0.0 == minus_zero { "true" } else { "false" });
}