use std::fmt::Write;

type Responder = fn(p1: i32) -> &'static str;

struct Delegate {
    operation: Option<Responder>,
}

impl Delegate {
    fn new(rspndr: Option<Responder>) -> Self {
        Delegate { operation: rspndr }
    }

    fn thing(&self, p1: i32) -> Option<&'static str> {
        self.operation.map(|op| op(p1))
    }
}

struct Delegator {
    param: i32,
    phrase: String,
    delegate: Delegate,
}

const fn default_response(p1: i32) -> &'static str {
    "default implementation"
}

impl Delegator {
    fn new(p: i32, phrase: &str) -> Self {
        Delegator {
            param: p,
            phrase: phrase.to_string(),
            delegate: Delegate::new(Some(default_response)),
        }
    }

    fn operation(&self, p1: i32, delroy: Option<&Delegate>) -> &'static str {
        let rtn = if let Some(delroy) = delroy {
            if let Some(response) = delroy.thing(p1) {
                response
            } else {
                self.delegate.thing(p1).unwrap_or(default_response(p1))
            }
        } else {
            self.delegate.thing(p1).unwrap_or(default_response(p1))
        };

        println!("{}", self.phrase);
        rtn
    }
}

fn thing1(p1: i32) -> &'static str {
    println!("We're in thing1 with value {}", p1);
    "delegate implementation"
}

fn main() {
    let del1 = Delegate::new(Some(thing1));
    let del2 = Delegate::new(None);
    let the_delegator = Delegator::new(14, "A stellar vista, Baby.");

    println!(
        "Delegator returns {}\n",
        the_delegator.operation(3, None)
    );
    println!(
        "Delegator returns {}\n",
        the_delegator.operation(3, Some(&del1))
    );
    println!(
        "Delegator returns {}\n",
        the_delegator.operation(3, Some(&del2))
    );
}