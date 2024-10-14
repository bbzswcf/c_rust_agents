use std::f64::consts::E;

type Class2Func = fn(f64) -> f64;

fn function_a(v: f64) -> f64 {
    v * v * v
}

fn function_b(v: f64) -> f64 {
    E.powf(v.ln() / 3.0)
}

fn function1(val: f64) -> f64 {
    // Modified: Call function_a instead of f2
    function_a(val)
}

fn which_func(idx: i32) -> Class2Func {
    // Modified: Correctly select the function based on the index
    if idx == 0 {
        function_a
    } else {
        function_b
    }
}

const FUNC_LIST_A: [Class2Func; 3] = [function_a, function_a, function_a]; // Modified: Added function_a
const FUNC_LIST_B: [Class2Func; 3] = [function_b, function_b, function_b]; // Modified: Added function_b

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
    // Modified: Correctly call the function pointers stored in the fields f1 and f2
    (comp.f1)((comp.f2)(val))
}

fn main() {
    // Modified: Correctly call function1 with function_a and 3.0 as arguments
    println!("Function1(functionA, 3.0) = {:.6}", function1(3.0));

    for ix in 0..3 {
        let c = compose(FUNC_LIST_A[ix], FUNC_LIST_B[ix]);
        // Modified: Correctly call call_composed with the correct composition and 0.9 as arguments
        println!("Composition {}(0.9) = {:.6}", ix, call_composed(&c, 0.9));
    }
}