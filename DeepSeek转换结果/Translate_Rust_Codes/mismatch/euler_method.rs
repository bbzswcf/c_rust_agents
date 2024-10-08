type DerivF = fn(f64, f64) -> f64;

const FMT: &str = " {:7.3}";

fn ivp_euler(f: DerivF, mut y: f64, step: i32, end_t: i32) {
    let mut t = 0;

    // 修改: 使用print!而不是println!来确保格式化正确
    print!(" Step {:2}:", step);
    loop {
        if t % 10 == 0 {
            // 修改: 使用print!而不是println!来确保格式化正确
            print!(" {:7.3}", y);
        }
        y += step as f64 * f(t as f64, y);
        t += step;
        if t > end_t {
            break;
        }
    }
    // 修改: 使用println!来添加换行符
    println!();
}

fn analytic() {
    print!("    Time: ");
    for t in (0..=100).step_by(10) {
        print!(" {:7}", t);
    }
    println!();
    print!("Analytic: ");

    for t in (0..=100).step_by(10) {
        // 修改: 使用print!而不是println!来确保格式化正确
        print!(" {:7.3}", 20.0 + 80.0 * std::f64::consts::E.powf(-0.07 * t as f64));
    }
    // 修改: 使用println!来添加换行符
    println!();
}

// 修改: 确保函数返回类型为f64
fn cooling(_t: f64, temp: f64) -> f64 {
    -0.07 * (temp - 20.0)
}

fn main() {
    analytic();
    ivp_euler(cooling, 100.0, 2, 100);
    ivp_euler(cooling, 100.0, 5, 100);
    ivp_euler(cooling, 100.0, 10, 100);
}