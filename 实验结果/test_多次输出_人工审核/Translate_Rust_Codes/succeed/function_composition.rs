use std::ops::Deref;

trait DoubleToDouble {
    fn call(&self, x: f64) -> f64;
}

struct ComposeFunctor<F: DoubleToDouble, G: DoubleToDouble> {
    f: F,
    g: G,
}

impl<F: DoubleToDouble, G: DoubleToDouble> DoubleToDouble for ComposeFunctor<F, G> {
    fn call(&self, x: f64) -> f64 {
        self.f.call(self.g.call(x))
    }
}

fn compose<F: DoubleToDouble, G: DoubleToDouble>(f: F, g: G) -> impl DoubleToDouble {
    ComposeFunctor { f, g }
}

struct Sin;

impl DoubleToDouble for Sin {
    fn call(&self, x: f64) -> f64 {
        x.sin()
    }
}

struct Asin;

impl DoubleToDouble for Asin {
    fn call(&self, x: f64) -> f64 {
        x.asin()
    }
}

fn main() {
    let my_sin = Sin;
    let my_asin = Asin;

    let sin_asin = compose(my_sin, my_asin);

    println!("{}", sin_asin.call(0.5)); // prints "0.500000"
}