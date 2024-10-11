// Removed: Unused import
// use std::f64::consts::PI;

struct DoubleToDouble {
    fn_ptr: fn(f64) -> f64,
}

impl DoubleToDouble {
    fn call(&self, x: f64) -> f64 {
        (self.fn_ptr)(x)
    }
}

struct ComposeFunctor {
    f: Box<DoubleToDouble>,
    g: Box<DoubleToDouble>,
}

impl ComposeFunctor {
    fn call(&self, x: f64) -> f64 {
        self.f.call(self.g.call(x))
    }
}

// Modified: Added `this` parameter to the function signature
fn compose_call(this: &dyn AsAny, x: f64) -> f64 {
    if let Some(compose_functor) = this.as_any().downcast_ref::<ComposeFunctor>() {
        compose_functor.call(x)
    } else {
        panic!("Expected a ComposeFunctor");
    }
}

trait AsAny {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl AsAny for DoubleToDouble {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

fn compose(f: Box<DoubleToDouble>, g: Box<DoubleToDouble>) -> Box<ComposeFunctor> {
    Box::new(ComposeFunctor { f, g })
}

fn sin_call(x: f64) -> f64 {
    x.sin()
}

fn asin_call(x: f64) -> f64 {
    x.asin()
}

fn main() {
    let my_sin = Box::new(DoubleToDouble { fn_ptr: sin_call });
    let my_asin = Box::new(DoubleToDouble { fn_ptr: asin_call });

    let sin_asin = compose(my_sin, my_asin);

    println!("{}", sin_asin.call(0.5)); // prints "0.500000"
}