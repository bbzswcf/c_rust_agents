use std::boxed::Box;

type Responder = fn(i32) -> &'static str;

struct Delegate {
    operation: Responder,
}

impl Delegate {
    fn new(rspndr: Responder) -> Box<Self> {
        Box::new(Delegate { operation: rspndr })
    }

    fn thing(&self, p1: i32) -> Option<&'static str> {
        // Modified: Directly call `self.operation` without wrapping it in `Some`
        Some((self.operation)(p1))
    }
}

struct Delegator {
    param: i32,
    phrase: String,
    delegate: Box<Delegate>,
}

const fn default_response(p1: i32) -> &'static str {
    "default implementation"
}

static DEFAULT_DEL: Delegate = Delegate { operation: default_response };

impl Delegator {
    fn new(p: i32, phrase: &str) -> Box<Self> {
        Box::new(Delegator {
            param: p,
            phrase: phrase.to_string(),
            // Modified: Create a new `Delegate` instance with the same `operation` field
            delegate: Box::new(Delegate { operation: DEFAULT_DEL.operation }),
        })
    }

    fn operation(&self, p1: i32, delroy: Option<&Delegate>) -> &'static str {
        let rtn = if let Some(delroy) = delroy {
            if let Some(result) = delroy.thing(p1) {
                result
            } else {
                self.delegate.thing(p1).unwrap_or_default()
            }
        } else {
            self.delegate.thing(p1).unwrap_or_default()
        };

        println!("{}", self.phrase);
        rtn
    }
}

const fn thing1(p1: i32) -> &'static str {
    // Modified: Removed `println!` call to make the function `const`
    "delegate implementation"
}

fn main() {
    let del1 = Delegate::new(thing1);
    // Modified: Provide a valid function pointer instead of `None`
    let del2 = Delegate::new(default_response);
    let the_delegator = Delegator::new(14, "A stellar vista, Baby.");

    println!("Delegator returns {}\n", the_delegator.operation(3, None));
    // Modified: Dereference `del1` correctly to get a `&Delegate`
    println!("Delegator returns {}\n", the_delegator.operation(3, Some(&*del1)));
    println!("Delegator returns {}\n", the_delegator.operation(3, Some(&*del2)));
}