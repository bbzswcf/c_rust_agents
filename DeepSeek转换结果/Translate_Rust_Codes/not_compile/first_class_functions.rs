// 修改: 添加lazy_static依赖到Cargo.toml
// [dependencies]
// lazy_static = "1.4.0"

use lazy_static::lazy_static;

type Class2Func = fn(f64) -> f64;

fn function_a(v: f64) -> f64 {
    v * v * v
}

fn function_b(v: f64) -> f64 {
    (v.ln() / 3.0).exp()
}

fn function1(f2: Class2Func, val: f64) -> f64 {
    f2(val)
}

fn which_func(idx: i32) -> Class2Func {
    if idx < 4 {
        function_a
    } else {
        function_b
    }
}

// 修改: 使用lazy_static!宏定义静态变量
lazy_static! {
    static ref FUNC_LIST_A: [Box<dyn Fn(f64) -> f64>; 4] = [
        Box::new(function_a),
        Box::new(|x| x.sin()),
        Box::new(|x| x.cos()),
        Box::new(|x| x.tan())
    ];

    static ref FUNC_LIST_B: [Box<dyn Fn(f64) -> f64>; 4] = [
        Box::new(function_b),
        Box::new(|x| x.asin()),
        Box::new(|x| x.acos()),
        Box::new(|x| x.atan())
    ];
}

fn invoke_composed(f1: Box<dyn Fn(f64) -> f64>, f2: Box<dyn Fn(f64) -> f64>, val: f64) -> f64 {
    f1(f2(val))
}

struct Composition {
    f1: Box<dyn Fn(f64) -> f64>,
    f2: Box<dyn Fn(f64) -> f64>,
}

// 修改: 使用Box<dyn Fn(f64) -> f64>作为参数类型
fn compose(f1: Box<dyn Fn(f64) -> f64>, f2: Box<dyn Fn(f64) -> f64>) -> Composition {
    Composition { f1, f2 }
}

// 修改: 调用组合函数
fn call_composed(comp: &Composition, val: f64) -> f64 {
    (comp.f1)((comp.f2)(val))
}

fn main() {
    // 修改: 移除lazy_static::initialize调用，因为lazy_static!宏会自动初始化

    println!("Function1(functionA, 3.0) = {}", function1(which_func(0), 3.0));

    for ix in 0..4 {
        // 修改: 使用lazy_static!宏定义的静态变量
        let c = compose(FUNC_LIST_A[ix].clone(), FUNC_LIST_B[ix].clone());
        println!("Compostion {}(0.9) = {}", ix, call_composed(&c, 0.9));
    }
}