fn rat_approx(f: f64, md: i64, num: &mut i64, denom: &mut i64) {
    // 修改: 将 new_d 和 new_n 初始化为 0
    let mut new_d: i64 = 0;
    let mut new_n: i64 = 0;

    // 修改: 将 h 和 k 初始化为 Vec<i64>
    let h = vec![0, 1, 0];
    let k = vec![1, 0, 0];
    let mut n = 1; // 修改: 将 n 初始化为 1
    let mut neg = 0; // 修改: 将 neg 初始化为 0

    if md <= 1 {
        *denom = 1;
        *num = f as i64;
        return;
    }

    let mut f = f;
    if f < 0.0 {
        neg = 1; // 修改: 将 neg 设置为 1
        f = -f;
    }

    while f != f.floor() {
        n <<= 1;
        f *= 2.0;
    }

    new_d = f as i64;
    new_n = n;

    for i in 0..64 {
        let a = if new_n != 0 { new_d / new_n } else { 0 };
        if i != 0 && a == 0 {
            break;
        }

        // 修改: 将 temp_d 初始化为 new_d
        let mut temp_d = new_d;
        new_d = new_n;
        new_n = temp_d % new_n;

        let temp_a = a;
        if k[1] * a + k[0] >= md {
            let x = (md - k[0]) / k[1];
            if x * 2 >= a || k[1] >= md {
                break;
            } else {
                break;
            }
        }

        // 修改: 将 h0 和 k0 初始化为 h[1] 和 k[1]
        let mut h0 = h[1];
        let mut k0 = k[1];

        // 修改: 将 h1 和 k1 初始化为 temp_a * h[1] + h[0] 和 temp_a * k[1] + k[0]
        let mut h1 = temp_a * h[1] + h[0];
        let mut k1 = temp_a * k[1] + k[0];

        h0 = h[1];
        k0 = k[1];
    }
    *denom = k[1];
    *num = if neg != 0 { -h[1] } else { h[1] };
}

fn main() {
    // 修改: 将 new_d 和 new_n 初始化为 0
    let mut new_d: i64 = 0;
    let mut new_n: i64 = 0;
    let f = 1.0 / 7.0;

    print!("f = {:.14}\n", f);
    for i in (1..=20000000).step_by(16) {
        print!("denom <= {}: ", i);
        rat_approx(f, i as i64, &mut new_n, &mut new_d);
        print!("{}/{}\n", new_n, new_d);
    }

    let f = f64::atan2(1.0, 1.0) * 4.0;
    print!("\nf = {:.14}\n", f);
    for i in (1..=20000000).step_by(16) {
        print!("denom <= {}: ", i);
        rat_approx(f, i as i64, &mut new_n, &mut new_d);
        print!("{}/{}\n", new_n, new_d);
    }
}