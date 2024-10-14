fn function_a(v: f64) -> f64 {
    v * v * v
}

fn function_b(v: f64) -> f64 {
    v.exp().ln() / 3.0
}

fn function1(f2: fn(f64) -> f64, val: f64) -> f64 {
    f2(val)
}

fn which_func(idx: i32) -> fn(f64) -> f64 {
    if idx < 4 {
        function_a
    } else {
        function_b
    }
}

fn invoke_composed(f1: fn(f64) -> f64, f2: fn(f64) -> f64, val: f64) -> f64 {
    f1(f2(val))
}

struct Composition {
    f1: fn(f64) -> f64,
    f2: fn(f64) -> f64,
}

fn compose(f1: fn(f64) -> f64, f2: fn(f64) -> f64) -> Box<Composition> {
    Box::new(Composition { f1, f2 })
}

fn call_composed(comp: &Composition, val: f64) -> f64 {
    (comp.f1)((comp.f2)(val))
}

fn main() {
    let func_list_a: [fn(f64) -> f64; 4] = [function_a, f64::sin, f64::cos, f64::tan];
    let func_list_b: [fn(f64) -> f64; 4] = [function_b, f64::asin, f64::acos, f64::atan];

    println!("Function1(functionA, 3.0) = {}", function1(which_func(0), 3.0));

    for (ix, (f1, f2)) in func_list_a.iter().zip(func_list_b.iter()).enumerate() {
        let c = compose(*f1, *f2);
        println!("Composition {}(0.9) = {}", ix, call_composed(&*c, 0.9));
    }
}