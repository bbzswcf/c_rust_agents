type Responder = fn(i32) -> &'static str;

#[derive(Clone)]
struct Delegate {
    operation: Option<Responder>,
}

impl Delegate {
    fn new(rspndr: Option<Responder>) -> Box<Self> {
        Box::new(Delegate { operation: rspndr })
    }

    fn thing(&self, p1: i32) -> Option<&'static str> {
        self.operation.map(|op| op(p1))
    }
}

struct Delegator {
    param: i32,
    phrase: String,
    delegate: Box<Delegate>,
}

const fn default_response(_p1: i32) -> &'static str {
    "default implementation"
}

static DEFAULT_DEL: Delegate = Delegate { operation: Some(default_response) };

impl Delegator {
    fn new(p: i32, phrase: String) -> Box<Self> {
        Box::new(Delegator {
            param: p,
            phrase: phrase,
            delegate: Box::new(DEFAULT_DEL.clone()),
        })
    }

    fn operation(&self, p1: i32, delroy: Option<&Delegate>) -> &'static str {
        let rtn = if let Some(delroy) = delroy {
            if let Some(result) = delroy.thing(p1) {
                result
            } else {
                self.delegate.thing(p1).unwrap_or_else(|| "default implementation")
            }
        } else {
            self.delegate.thing(p1).unwrap_or_else(|| "default implementation")
        };

        println!("{}", self.phrase);
        rtn
    }
}

fn thing1(p1: i32) -> &'static str {
    print!("We're in thing1 with value {}\n", p1);
    "delegate implementation"
}

fn main() {
    let del1 = Delegate::new(Some(thing1));
    let del2 = Delegate::new(None);
    let the_delegator = Delegator::new(14, "A stellar vista, Baby.".to_string());

    println!("Delegator returns {}\n", the_delegator.operation(3, None));
    println!("Delegator returns {}\n", the_delegator.operation(3, Some(&*del1)));
    println!("Delegator returns {}\n", the_delegator.operation(3, Some(&*del2)));
}