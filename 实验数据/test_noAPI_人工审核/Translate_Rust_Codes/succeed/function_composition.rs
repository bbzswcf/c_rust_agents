use std::boxed::Box;

trait DoubleToDouble {
    fn call(&self, x: f64) -> f64;
}

struct ComposeFunctor {
    f: Box<dyn DoubleToDouble>,
    g: Box<dyn DoubleToDouble>,
}

impl DoubleToDouble for ComposeFunctor {
    fn call(&self, x: f64) -> f64 {
        self.f.call(self.g.call(x))
    }
}

fn compose(f: Box<dyn DoubleToDouble>, g: Box<dyn DoubleToDouble>) -> Box<dyn DoubleToDouble> {
    Box::new(ComposeFunctor { f, g })
}

struct SinFunctor;

impl DoubleToDouble for SinFunctor {
    fn call(&self, x: f64) -> f64 {
        x.sin()
    }
}

struct AsinFunctor;

impl DoubleToDouble for AsinFunctor {
    fn call(&self, x: f64) -> f64 {
        x.asin()
    }
}

fn main() {
    let my_sin: Box<dyn DoubleToDouble> = Box::new(SinFunctor);
    let my_asin: Box<dyn DoubleToDouble> = Box::new(AsinFunctor);

    let sin_asin = compose(my_sin, my_asin);

    println!("{}", sin_asin.call(0.5)); // prints "0.500000"
}