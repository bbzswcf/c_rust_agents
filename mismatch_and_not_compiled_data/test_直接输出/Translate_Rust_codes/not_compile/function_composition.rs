struct DoubleToDouble {
    fn: fn(&DoubleToDouble, f64) -> f64,
}

struct ComposeFunctor {
    fn: fn(&ComposeFunctor, f64) -> f64,
    f: Box<DoubleToDouble>,
    g: Box<DoubleToDouble>,
}

fn compose_call(this: &ComposeFunctor, x: f64) -> f64 {
    (this.f.fn)(&*this.f, (this.g.fn)(&*this.g, x))
}

fn compose(f: Box<DoubleToDouble>, g: Box<DoubleToDouble>) -> Box<DoubleToDouble> {
    let result = Box::new(ComposeFunctor {
        fn: compose_call,
        f,
        g,
    });
    Box::new(DoubleToDouble { fn: |this, x| (result.fn)(&*result, x) })
}

fn sin_call(_: &DoubleToDouble, x: f64) -> f64 {
    x.sin()
}

fn asin_call(_: &DoubleToDouble, x: f64) -> f64 {
    x.asin()
}

fn main() {
    let my_sin = Box::new(DoubleToDouble { fn: sin_call });
    let my_asin = Box::new(DoubleToDouble { fn: asin_call });

    let sin_asin = compose(my_sin, my_asin);

    println!("{}", (sin_asin.fn)(&*sin_asin, 0.5)); // prints "0.500000"
}