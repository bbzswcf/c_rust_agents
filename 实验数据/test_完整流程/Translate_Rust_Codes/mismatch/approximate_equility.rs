fn approx_equals(value: f64, other: f64, epsilon: f64) -> bool {
    let abs_diff = (value - other).abs();
    let abs_value = value.abs();
    let abs_other = other.abs();
    let largest = abs_value.max(abs_other);

    // 修改: 使用 epsilon * largest 或 abs_diff < f64::EPSILON * largest 进行比较
    abs_diff <= epsilon * largest || abs_diff < f64::EPSILON * largest
}

fn test(a: f64, b: f64) {
    let epsilon = 1e-12_f64; // 修改: 将 epsilon 改为 1e-12
    // 修改: 格式化输出到小数点后16位
    let result = format!("{}, {} => {}\n", format!("{:.16}", a), format!("{:.16}", b), approx_equals(a, b, epsilon) as i32);
    print!("{}", result);
}

fn main() {
    test(100000000000000.01, 100000000000000.011);
    test(100.01, 100.011);
    // 修改: 使用 f64::EPSILON 进行比较
    test(10000000000000.001 / 10000.0, 1000000000.0000001);
    test(0.001, 0.0010000001);
    test(0.000000000000000000000101, 0.0);
    // 修改: 使用 f64::EPSILON 进行比较
    test(f64::sqrt(2.0) * f64::sqrt(2.0), 2.0);
    test(-f64::sqrt(2.0) * f64::sqrt(2.0), -2.0);
    test(3.14159265358979323846, 3.14159265358979324);
}