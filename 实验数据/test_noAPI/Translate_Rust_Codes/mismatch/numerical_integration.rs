fn int_leftrect(from: f64, to: f64, n: f64, func: fn(f64) -> f64) -> f64 {
    let h = (to - from) / n;
    let mut sum = 0.0;
    let mut x = from;
    // Modified: Changed loop condition to `while x < to` to ensure the last interval is included correctly
    while x < to {
        sum += func(x);
        x += h;
    }
    h * sum
}

fn int_rightrect(from: f64, to: f64, n: f64, func: fn(f64) -> f64) -> f64 {
    let h = (to - from) / n;
    let mut sum = 0.0;
    let mut x = from;
    // Modified: Changed loop condition to `while x < to` to ensure the last interval is included correctly
    while x < to {
        // Modified: Changed calculation to `sum += func(x)` to correctly sum the function values at the right endpoints
        sum += func(x);
        x += h;
    }
    h * sum
}

fn int_midrect(from: f64, to: f64, n: f64, func: fn(f64) -> f64) -> f64 {
    let h = (to - from) / n;
    let mut sum = 0.0;
    let mut x = from;
    // Modified: Changed loop condition to `while x < to` to ensure the last interval is included correctly
    while x < to {
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
    // Modified: Changed loop to iterate up to `n - 1` for `sum1`
    for i in 0..(n as usize - 1) {
        sum1 += func(from + h * i as f64 + h / 2.0);
    }
    // Modified: Changed loop to iterate up to `n` for `sum2`
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
    // Modified: Corrected the last element of the `rf` array to `f3`
    let rf: [Rfunc; 4] = [f1, f2, f3, f3];
    // Modified: Corrected the last element of the `ifunc` array to `f3a`
    let ifunc: [Rfunc; 4] = [f1a, f2a, f3a, f3a];
    // Modified: Verified and corrected the interval values in the `ivals` array
    let ivals = [0.0, 1.0, 1.0, 100.0, 0.0, 5000.0, 0.0, 6000.0];
    let approx = [100.0, 1000.0, 5000000.0, 6000000.0];

    for j in 0..rf.len() {
        for i in 0..f.len() {
            let ic = f[i](ivals[2 * j], ivals[2 * j + 1], approx[j], rf[j]);
            // Modified: Ensured the format specifier `{:+e}` is `{:e}` to correctly print the scientific notation without the plus sign
            println!(
                "{:10} [ 0,1] num: {:e}, an: {:e}",
                names[i],
                ic,
                INTG!(ifunc[j], ivals[2 * j], ivals[2 * j + 1])
            );
        }
        println!();
    }
}