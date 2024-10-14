use std::ptr;
use std::fmt;

type Responder = fn(i32) -> Option<String>;

// Modified: Ensure the Clone trait is only derived once
#[derive(Clone)]
struct Delegate {
    operation: Responder,
}

impl Delegate {
    fn new(rspndr: Responder) -> Box<Self> {
        Box::new(Delegate { operation: rspndr })
    }

    // Modified: Directly return the result of the function pointer without wrapping it in an Option
    fn thing(&self, p1: i32) -> Option<String> {
        (self.operation)(p1)
    }
}

struct Delegator {
    param: i32,
    phrase: String,
    delegate: Box<Delegate>,
}

fn default_response(p1: i32) -> Option<String> {
    Some("default implementation".to_string())
}

static DEFAULT_DEL: Delegate = Delegate { operation: default_response };

impl Delegator {
    fn new(p: i32, phrase: String) -> Box<Self> {
        Box::new(Delegator {
            param: p,
            phrase,
            // Modified: Clone the DEFAULT_DEL to avoid moving out of a static item
            delegate: Box::new(DEFAULT_DEL.clone()),
        })
    }

    // Modified: Simplify the logic by directly calling delroy.thing(p1)
    fn operation(&self, p1: i32, delroy: Option<&Box<Delegate>>) -> Option<String> {
        let rtn = delroy.map_or_else(|| self.delegate.thing(p1), |delroy| delroy.thing(p1));
        println!("{}", self.phrase);
        rtn
    }
}

fn thing1(p1: i32) -> Option<String> {
    println!("We're in thing1 with value {}", p1);
    Some("delegate implementation".to_string())
}

// Modified: Define a no-op function to handle None case
fn no_op(_: i32) -> Option<String> { None }

fn main() {
    let del1 = Delegate::new(thing1);
    // Modified: Use default_response instead of no_op when creating del2
    let del2 = Delegate::new(default_response);
    let the_delegator = Delegator::new(14, "A stellar vista, Baby.".to_string());

    // Modified: Remove unwrap_or as it is not needed
    println!("Delegator returns {:?}\n", the_delegator.operation(3, None));
    println!("Delegator returns {:?}\n", the_delegator.operation(3, Some(&del1)));
    println!("Delegator returns {:?}\n", the_delegator.operation(3, Some(&del2)));
}