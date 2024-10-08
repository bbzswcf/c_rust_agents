fn main() {
    let mut e: f64;

    println!("The double precision in C give about 15 significant digits.\n\
              Values below are presented with 16 digits after the decimal point.\n");

    // The most direct way to compute Euler constant.
    //
    e = std::f64::consts::E;
    print!("Euler constant e = {:.16}\n", e);

    // The fast and independed method: e = lim (1 + 1/n)**n
    //
    let n: i32 = 8192;
    e = 1.0 + 1.0 / (n as f64);
    for _ in 0..13 {
        e *= e;
    }
    print!("Euler constant e = {:.16}\n", e);

    // Taylor expansion e = 1 + 1/1 + 1/2 + 1/2/3 + 1/2/3/4 + 1/2/3/4/5 + ...
    // Actually Kahan summation may improve the accuracy, but is not necessary.
    //
    const N: usize = 1000;
    let mut a = [0.0; N];
    a[0] = 1.0;
    for i in 1..N {
        a[i] = a[i - 1] / (i as f64);
    }
    e = 1.0;
    for i in (1..N).rev() {
        e += a[i];
    }
    print!("Euler constant e = {:.16}\n", e);
}