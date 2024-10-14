fn mean(v: &[f64]) -> f64 {
    let sum: f64 = v.iter().sum();
    // 修改: 当数组长度为0时返回NaN
    if v.len() == 0 {
        return f64::NAN; // NaN
    }
    sum / v.len() as f64
}

fn main() {
    // 修改: 初始化数组
    let v = [1.0, 2.0, 2.718, 3.0, 3.142];
    for len in (0..=5).rev() {
        print!("mean[");
        for (i, &val) in v.iter().enumerate().take(len) {
            if i > 0 {
                print!(", {}", val);
            } else {
                print!("{}", val);
            }
        }
        let mean_value = mean(&v[..len]);
        // 修改: 当mean_value为NaN时输出-nan
        if mean_value.is_nan() {
            println!("] = -nan"); // -nan
        } else {
            println!("] = {}", mean_value); // 不使用格式化
        }
    }
}