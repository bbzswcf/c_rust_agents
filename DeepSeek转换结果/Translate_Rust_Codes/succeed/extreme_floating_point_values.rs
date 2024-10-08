fn main() {
    let inf: f64 = f64::INFINITY;
    let minus_inf: f64 = f64::NEG_INFINITY;
    let minus_zero: f64 = -0.0;
    let nan: f64 = f64::NAN;

    println!("positive infinity: {}", inf);
    println!("negative infinity: {}", minus_inf);
    println!("negative zero: {:.6}", minus_zero);
    println!("not a number: {}", if nan.is_nan() { "-nan" } else { "nan" });

    /* some arithmetic */

    println!("+inf + 2.0 = {}", inf + 2.0);
    println!("+inf - 10.1 = {}", inf - 10.1);
    println!("+inf + -inf = {}", if (inf + minus_inf).is_nan() { "-nan" } else { "nan" });
    println!("0.0 * +inf = {}", if (0.0 * inf).is_nan() { "-nan" } else { "nan" });
    println!("1.0/-0.0 = {}", 1.0 / minus_zero);
    println!("NaN + 1.0 = {}", if (nan + 1.0).is_nan() { "-nan" } else { "nan" });
    println!("NaN + NaN = {}", if (nan + nan).is_nan() { "-nan" } else { "nan" });

    /* some comparisons */

    println!("NaN == NaN = {}", if nan.is_nan() { "false" } else { "true" });
    println!("0.0 == -0.0 = {}", if 0.0 == minus_zero { "true" } else { "false" });
}