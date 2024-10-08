struct Test {
    a: i32,
    b: i32,
    c: i32,
}

// 使用标准库的swap函数
fn swap<T>(a: &mut T, b: &mut T) {
    std::mem::swap(a, b);
}

fn main() {
    let mut t = Test { a: 1, b: 2, c: 3 };
    let mut h = Test { a: 4, b: 5, c: 6 };
    let mut alfa = 0.45f64;
    let mut omega = 9.98f64;

    // 打印t的值
    print!("{} {} {}\n", t.a, t.b, t.c);
    swap(&mut t, &mut h); // 交换t和h
    print!("{} {} {}\n", t.a, t.b, t.c);
    print!("{} {} {}\n", h.a, h.b, h.c);

    // 修改: 将格式化字符串改为{:.6}以保留6位小数
    print!("{:.6}\n", alfa);
    swap(&mut alfa, &mut omega); // 交换alfa和omega
    print!("{:.6}\n", alfa);

    // 打印t的值
    print!("{}\n", t.a);
    swap(&mut t, &mut h); // 交换t和h
    print!("{}\n", t.a);
}