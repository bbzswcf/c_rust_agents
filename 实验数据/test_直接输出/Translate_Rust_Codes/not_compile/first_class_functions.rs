use std::f64::consts::E;

type Class2Func = fn(f64) -> f64;

fn function_a(v: f64) -> f64 {
    v * v * v
}

fn function_b(v: f64) -> f64 {
    E.powf(v.ln() / 3.0)
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

const FUNC_LIST_A: [Class2Func; 4] = [function_a, f64::sin, f64::cos, f64::tan];
const FUNC_LIST_B: [Class2Func; 4] = [function_b, f64::asin, f64::acos, f64::atan];

fn invoke_composed(f1: Class2Func, f2: Class2Func, val: f64) -> f64 {
    f1(f2(val))
}

struct Composition {
    f1: Class2Func,
    f2: Class2Func,
}

fn compose(f1: Class2Func, f2: Class2Func) -> Box<Composition> {
    Box::new(Composition { f1, f2 })
}

fn call_composed(comp: &Composition, val: f64) -> f64 {
    comp.f1(comp.f2(val))
}

fn main() {
    println!("Function1(functionA, 3.0) = {}", function1(which_func(0), 3.0));

    for ix in 0..4 {
        let c = compose(FUNC_LIST_A[ix], FUNC_LIST_B[ix]);
        println!("Compostion {}(0.9) = {}", ix, call_composed(&c, 0.9));
    }
}