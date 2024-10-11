struct DoubleToDouble {
    fn: fn(&DoubleToDouble, f64) -> f64,
}

impl DoubleToDouble {
    fn call(&self, x: f64) -> f64 {
        (self.fn)(self, x)
    }
}

struct ComposeFunctor {
    fn: fn(&ComposeFunctor, f64) -> f64,
    f: Box<DoubleToDouble>,
    g: Box<DoubleToDouble>,
}

impl ComposeFunctor {
    fn call(&self, x: f64) -> f64 {
        (self.fn)(self, x)
    }
}

fn compose_call(this: &ComposeFunctor, x: f64) -> f64 {
    this.f.call(this.g.call(x))
}

fn compose(f: Box<DoubleToDouble>, g: Box<DoubleToDouble>) -> Box<DoubleToDouble> {
    let result = ComposeFunctor {
        fn: compose_call,
        f,
        g,
    };
    Box::new(DoubleToDouble {
        fn: |this, x| result.call(x),
    })
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

    println!("{}", sin_asin.call(0.5)); // prints "0.500000"
}