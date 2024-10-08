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
    // Modified: Corrected function selection logic
    if idx < 4 {
        function_b
    } else {
        function_a
    }
}

// Modified: Defined new functions to match the Class2Func type signature
fn sin_func(v: f64) -> f64 {
    v.sin()
}
fn cos_func(v: f64) -> f64 {
    v.cos()
}
fn tan_func(v: f64) -> f64 {
    v.tan()
}
fn asin_func(v: f64) -> f64 {
    v.asin()
}
fn acos_func(v: f64) -> f64 {
    v.acos()
}
fn atan_func(v: f64) -> f64 {
    v.atan()
}

// Modified: Updated FUNC_LIST_A and FUNC_LIST_B to use the new functions
const FUNC_LIST_A: [Class2Func; 4] = [function_b, asin_func, acos_func, atan_func];
const FUNC_LIST_B: [Class2Func; 4] = [function_a, sin_func, cos_func, tan_func];

fn invoke_composed(f1: Class2Func, f2: Class2Func, val: f64) -> f64 {
    f1(f2(val))
}

struct Composition {
    f1: Class2Func,
    f2: Class2Func,
}

fn compose(f1: Class2Func, f2: Class2Func) -> Composition {
    Composition { f1, f2 }
}

fn call_composed(comp: &Composition, val: f64) -> f64 {
    // Modified: Corrected function composition logic
    (comp.f1)((comp.f2)(val)) // Corrected: Use parentheses around field access for function pointers
}

// Modified: Defined the Mouth struct and its implementation before usage
struct Mouth;

impl Mouth {
    fn chocolate(&self) {
        println!("Hmmm! I love chocolate!");
    }
}

fn main() {
    // Modified: Corrected function call to directly call function_a
    println!("Function1(functionA, 3.0) = {}", function1(function_a, 3.0));

    for ix in 0..4 {
        // Modified: Corrected function list usage for composition
        let c = compose(FUNC_LIST_A[ix], FUNC_LIST_B[ix]);
        println!("Compostion {}(0.9) = {:.6}", ix, call_composed(&c, 0.9)); // Modified: Ensured consistent floating-point precision
    }

    // Modified: Corrected method call syntax for the Mouth struct
    let x = Mouth;
    x.chocolate();
}