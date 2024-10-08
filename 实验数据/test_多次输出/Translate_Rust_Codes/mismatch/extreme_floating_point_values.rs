fn main() {
    let inf: f64 = 1.0 / 0.0;
    let minus_inf: f64 = -1.0 / 0.0;
    let minus_zero: f64 = -1.0 / inf;
    let nan: f64 = 0.0 / 0.0;

    println!("positive infinity: {}", inf);
    println!("negative infinity: {}", minus_inf);
    println!("negative zero: {}", minus_zero);
    println!("not a number: {}", nan);

    // Some arithmetic
    println!("+inf + 2.0 = {}", inf + 2.0);
    println!("+inf - 10.1 = {}", inf - 10.1);
    println!("+inf + -inf = {}", inf + minus_inf);
    println!("0.0 * +inf = {}", 0.0 * inf);
    println!("1.0/-0.0 = {}", 1.0 / minus_zero);
    println!("NaN + 1.0 = {}", nan + 1.0);
    println!("NaN + NaN = {}", nan + nan);

    // Some comparisons
    println!("NaN == NaN = {}", nan == nan);
    println!("0.0 == -0.0 = {}", 0.0 == minus_zero);
}