fn int_leftrect<F>(from: f64, to: f64, n: f64, func: F) -> f64
where
    F: Fn(f64) -> f64,
{
    let h = (to - from) / n;
    let mut sum = 0.0;
    let mut x = from;
    while x <= to - h {
        sum += func(x);
        x += h;
    }
    h * sum
}

fn int_rightrect<F>(from: f64, to: f64, n: f64, func: F) -> f64
where
    F: Fn(f64) -> f64,
{
    let h = (to - from) / n;
    let mut sum = 0.0;
    let mut x = from;
    while x <= to - h {
        sum += func(x + h);
        x += h;
    }
    h * sum
}

fn int_midrect<F>(from: f64, to: f64, n: f64, func: F) -> f64
where
    F: Fn(f64) -> f64,
{
    let h = (to - from) / n;
    let mut sum = 0.0;
    let mut x = from;
    while x <= to - h {
        sum += func(x + h / 2.0);
        x += h;
    }
    h * sum
}

fn int_trapezium<F>(from: f64, to: f64, n: f64, func: F) -> f64
where
    F: Fn(f64) -> f64,
{
    let h = (to - from) / n;
    let mut sum = func(from) + func(to);
    for i in 1..n as i32 {
        sum += 2.0 * func(from + i as f64 * h);
    }
    h * sum / 2.0
}

fn int_simpson<F>(from: f64, to: f64, n: f64, func: F) -> f64
where
    F: Fn(f64) -> f64,
{
    let h = (to - from) / n;
    let mut sum1 = 0.0;
    let mut sum2 = 0.0;
    for i in 0..n as i32 {
        sum1 += func(from + h * i as f64 + h / 2.0);
    }
    for i in 1..n as i32 {
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

type Integrator = fn(f64, f64, f64, fn(f64) -> f64) -> f64;
type Func = fn(f64) -> f64;

fn main() {
    let integrators: [Integrator; 5] = [
        int_leftrect,
        int_rightrect,
        int_midrect,
        int_trapezium,
        int_simpson,
    ];
    let names = [
        "leftrect", "rightrect", "midrect", "trapezium", "simpson",
    ];
    let funcs: [Func; 4] = [f1, f2, f3, f3];
    let antiderivatives: [Func; 4] = [f1a, f2a, f3a, f3a];
    let intervals = [
        0.0, 1.0, 1.0, 100.0, 0.0, 5000.0, 0.0, 6000.0,
    ];
    let approximations = [
        100.0, 1000.0, 5000000.0, 6000000.0,
    ];

    for (j, &func) in funcs.iter().enumerate() {
        for (i, &integrator) in integrators.iter().enumerate() {
            let ic = integrator(
                intervals[2 * j],
                intervals[2 * j + 1],
                approximations[j],
                func,
            );
            let exact = antiderivatives[j](intervals[2 * j + 1]) - antiderivatives[j](intervals[2 * j]);
            println!(
                "{:10} [ 0,1] num: {:+lf}, an: {:lf}",
                names[i], ic, exact
            );
        }
        println!();
    }
}