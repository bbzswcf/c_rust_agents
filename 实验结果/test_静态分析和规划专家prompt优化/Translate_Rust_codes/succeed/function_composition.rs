use std::f64::consts::PI;

struct DoubleToDouble {
    fn_ptr: fn(&DoubleToDouble, f64) -> f64,
}

impl DoubleToDouble {
    fn call(&self, x: f64) -> f64 {
        (self.fn_ptr)(self, x)
    }
}

struct ComposeFunctor {
    fn_ptr: fn(&ComposeFunctor, f64) -> f64,
    f: Box<DoubleToDouble>,
    g: Box<DoubleToDouble>,
}

impl ComposeFunctor {
    fn call(&self, x: f64) -> f64 {
        (self.fn_ptr)(self, x)
    }
}

fn compose_call(this: &ComposeFunctor, x: f64) -> f64 {
    this.f.call(this.g.call(x))
}

fn compose(f: Box<DoubleToDouble>, g: Box<DoubleToDouble>) -> Box<ComposeFunctor> {
    Box::new(ComposeFunctor {
        fn_ptr: compose_call as fn(&ComposeFunctor, f64) -> f64,
        f,
        g,
    })
}

fn sin_call(_: &DoubleToDouble, x: f64) -> f64 {
    x.sin()
}

fn asin_call(_: &DoubleToDouble, x: f64) -> f64 {
    x.asin()
}

fn main() {
    let my_sin = Box::new(DoubleToDouble { fn_ptr: sin_call });
    let my_asin = Box::new(DoubleToDouble { fn_ptr: asin_call });
    let sin_asin = compose(my_sin, my_asin);
    println!("{}", sin_asin.call(0.5)); // prints "0.500000"
}