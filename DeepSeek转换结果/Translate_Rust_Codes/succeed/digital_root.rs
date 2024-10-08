fn droot(x: i64, base: i32, pers: &mut Option<&mut i32>) -> i32 {
    let mut d = 0;
    // 修改: 使用 `as_mut()` 获取 `Option<&mut i32>` 的可变引用
    if let Some(pers_ref) = pers.as_mut() {
        // 修改: 解引用 `pers_ref` 以获取 `i32` 类型的值
        **pers_ref = 0;
        let mut x = x;
        while x >= base as i64 {
            let mut d_inner = 0;
            while x > 0 {
                d_inner += (x % base as i64) as i32;
                x /= base as i64;
            }
            // 修改: 将 `d_inner` 赋值给 `d`
            d = d_inner;
            x = d as i64;
            // 修改: 解引用 `pers_ref` 以获取 `i32` 类型的值
            **pers_ref += 1;
        }
    } else if x > 0 {
        // 修改: 计算 `d` 的值
        d = (x % (base - 1) as i64) as i32;
        if d == 0 {
            d = base - 1;
        }
    }
    d
}

fn main() {
    let x: [i64; 4] = [627615, 39390, 588225, 393900588225];
    for i in 0..4 {
        let mut pers = 0;
        // 修改: 将 `pers` 的可变引用传递给 `droot`
        let d = droot(x[i], 10, &mut Some(&mut pers));
        println!("{}: pers {}, root {}", x[i], pers, d);
    }
}