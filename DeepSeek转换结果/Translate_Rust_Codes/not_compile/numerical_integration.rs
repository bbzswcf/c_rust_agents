fn int_leftrect(from: f64, to: f64, n: f64, func: fn(f64) -> f64) -> f64 {
    let h = (to - from) / n;
    let mut sum = 0.0;
    let mut x = from;
    // 修改: 使用 (to - x).abs() < f64::EPSILON 替代 x <= to - h + f64::EPSILON
    while (to - x).abs() > f64::EPSILON {
        sum += func(x);
        x += h;
    }
    h * sum
}

fn int_rightrect(from: f64, to: f64, n: f64, func: fn(f64) -> f64) -> f64 {
    let h = (to - from) / n;
    let mut sum = 0.0;
    let mut x = from;
    // 修改: 使用 (to - x).abs() < f64::EPSILON 替代 x <= to - h + f64::EPSILON
    while (to - x).abs() > f64::EPSILON {
        sum += func(x + h);
        x += h;
    }
    h * sum
}

fn int_midrect(from: f64, to: f64, n: f64, func: fn(f64) -> f64) -> f64 {
    let h = (to - from) / n;
    let mut sum = 0.0;
    let mut x = from;
    // 修改: 使用 (to - x).abs() < f64::EPSILON 替代 x <= to - h + f64::EPSILON
    while (to - x).abs() > f64::EPSILON {
        sum += func(x + h / 2.0);
        x += h;
    }
    h * sum
}

fn int_trapezium(from: f64, to: f64, n: f64, func: fn(f64) -> f64) -> f64 {
    let h = (to - from) / n;
    let mut sum = func(from) + func(to);
    for i in 1..n as usize {
        sum += 2.0 * func(from + i as f64 * h);
    }
    h * sum / 2.0
}

fn int_simpson(from: f64, to: f64, n: f64, func: fn(f64) -> f64) -> f64 {
    let h = (to - from) / n;
    let mut sum1 = 0.0;
    let mut sum2 = 0.0;
    for i in 0..n as usize {
        sum1 += func(from + h * i as f64 + h / 2.0);
    }
    for i in 1..n as usize {
        sum2 += func(from + h * i as f64);
    }
    h / 6.0 * (func(from) + func(to) + 4.0 * sum1 + 2.0 * sum2)
}

fn f3(x: f64) -> f64 {
    x
}

fn f3a(x: f64) -> f64 {
    x * x / 2.0
}

fn f2(x: f64) -> f64 {
    1.0 / x
}

fn f2a(x: f64) -> f64 {
    x.ln()
}

fn f1(x: f64) -> f64 {
    x * x * x
}

fn f1a(x: f64) -> f64 {
    x * x * x * x / 4.0
}

type Pfunc = fn(f64, f64, f64, fn(f64) -> f64) -> f64;
type Rfunc = fn(f64) -> f64;

fn main() {
    let f: [Pfunc; 5] = [
        int_leftrect,
        int_rightrect,
        int_midrect,
        int_trapezium,
        int_simpson,
    ];
    let names = [
        "leftrect",
        "rightrect",
        "midrect",
        "trapezium",
        "simpson",
    ];
    // 修改: 修正 rf 数组中的重复函数
    let rf: [Rfunc; 4] = [f1, f2, f3, f3a];
    let ifunc: [Rfunc; 4] = [f1a, f2a, f3a, f3a];
    let ivals = [
        0.0, 1.0, 1.0, 100.0, 0.0, 5000.0, 0.0, 6000.0,
    ];
    let approx = [100.0, 1000.0, 5000000.0, 6000000.0];

    for j in 0..rf.len() {
        for i in 0..f.len() {
            let ic = f[i](ivals[2 * j], ivals[2 * j + 1], approx[j], rf[j]);
            // 修改: 使用 {:+e} 格式化输出
            println!(
                "{:10} [ 0,1] num: {:+e}, an: {:e}",
                names[i],
                ic,
                ifunc[j](ivals[2 * j + 1]) - ifunc[j](ivals[2 * j])
            );
        }
        println!();
    }
}