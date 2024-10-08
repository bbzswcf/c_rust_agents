fn int_leftrect(from: f64, to: f64, n: usize, func: fn(f64) -> f64) -> f64 {
    let h = (to - from) / n as f64;
    let mut sum = 0.0;
    let mut x = from;
    while x <= to - h {
        sum += func(x);
        x += h;
    }
    h * sum
}

fn int_rightrect(from: f64, to: f64, n: usize, func: fn(f64) -> f64) -> f64 {
    let h = (to - from) / n as f64;
    let mut sum = 0.0;
    let mut x = from;
    while x <= to - h {
        sum += func(x + h);
        x += h;
    }
    h * sum
}

fn int_midrect(from: f64, to: f64, n: usize, func: fn(f64) -> f64) -> f64 {
    let h = (to - from) / n as f64;
    let mut sum = 0.0;
    let mut x = from;
    while x <= to - h {
        sum += func(x + h / 2.0);
        x += h;
    }
    h * sum
}

fn int_trapezium(from: f64, to: f64, n: usize, func: fn(f64) -> f64) -> f64 {
    let h = (to - from) / n as f64;
    let mut sum = func(from) + func(to);
    for i in 1..n {
        sum += 2.0 * func(from + i as f64 * h);
    }
    h * sum / 2.0
}

fn int_simpson(from: f64, to: f64, n: usize, func: fn(f64) -> f64) -> f64 {
    let h = (to - from) / n as f64;
    let mut sum1 = 0.0;
    let mut sum2 = 0.0;
    for i in 0..n {
        sum1 += func(from + h * i as f64 + h / 2.0);
    }
    for i in 1..n {
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

type Pfunc = fn(f64, f64, usize, fn(f64) -> f64) -> f64;
type Rfunc = fn(f64) -> f64;

macro_rules! INTG {
    ($F:expr, $A:expr, $B:expr) => {
        $F($B) - $F($A)
    };
}

fn main() {
    let f: [Pfunc; 5] = [
        int_leftrect,
        int_rightrect,
        int_midrect,
        int_trapezium,
        int_simpson,
    ];
    let names = ["leftrect", "rightrect", "midrect", "trapezium", "simpson"];
    let rf: [Rfunc; 4] = [f1, f2, f3, f3]; // Modified: Corrected the function used for `f3` in `rf` array
    let ifunc: [Rfunc; 4] = [f1a, f2a, f3a, f3a]; // Modified: Corrected the function used for `f3a` in `ifunc` array
    let ivals = [0.0, 1.0, 1e-10, 1.0, 0.0, 5000.0, 0.0, 6000.0]; // Modified: Corrected the integration range to avoid division by zero
    let approx = [100, 1000, 5000000, 6000000]; // Modified: Corrected the number of iterations

    for j in 0..rf.len() {
        for i in 0..f.len() {
            let ic = f[i](ivals[2 * j], ivals[2 * j + 1], approx[j], rf[j]);
            println!(
                "{:10} [ {:+e}, {:+e} ] num: {:+.10e}, an: {:.10e}", // Modified: Corrected the format trait for floating-point numbers
                names[i],
                ivals[2 * j],
                ivals[2 * j + 1],
                ic,
                INTG!(ifunc[j], ivals[2 * j], ivals[2 * j + 1]) // Modified: Corrected the calculation of the exact integral value
            );
        }
        println!();
    }
}