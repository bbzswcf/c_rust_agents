struct DoubleToDouble {
    r#fn: fn(&DoubleToDouble, f64) -> f64,
}

struct ComposeFunctor {
    r#fn: fn(&ComposeFunctor, f64) -> f64,
    f: Box<DoubleToDouble>,
    g: Box<DoubleToDouble>,
}

fn compose_call(this: &ComposeFunctor, x: f64) -> f64 {
    (this.f.r#fn)(&*this.f, (this.g.r#fn)(&*this.g, x))
}

fn compose(f: Box<DoubleToDouble>, g: Box<DoubleToDouble>) -> Box<ComposeFunctor> {
    let result = ComposeFunctor {
        r#fn: compose_call,
        f,
        g,
    };
    Box::new(result)
}

fn sin_call(_this: &DoubleToDouble, x: f64) -> f64 {
    x.sin()
}

fn asin_call(_this: &DoubleToDouble, x: f64) -> f64 {
    x.asin()
}

fn main() {
    let my_sin = Box::new(DoubleToDouble { r#fn: sin_call });
    let my_asin = Box::new(DoubleToDouble { r#fn: asin_call });

    let sin_asin = compose(my_sin, my_asin);

    print!("{:.6}\n", (sin_asin.r#fn)(&*sin_asin, 0.5)); // prints "0.500000"
}