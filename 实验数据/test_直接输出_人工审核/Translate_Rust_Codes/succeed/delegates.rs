use std::boxed::Box;
use std::option::Option;

type Responder = fn(i32) -> &'static str;

struct SDelegate {
    operation: Option<Responder>,
}

struct Delegate {
    delegate: Box<SDelegate>,
}

impl Delegate {
    fn new(rspndr: Option<Responder>) -> Self {
        Delegate {
            delegate: Box::new(SDelegate { operation: rspndr }),
        }
    }

    fn thing(&self, p1: i32) -> Option<&'static str> {
        match &self.delegate.operation {
            Some(op) => Some(op(p1)),
            None => None,
        }
    }
}

struct SDelegator {
    param: i32,
    phrase: String,
    delegate: Delegate,
}

struct Delegator {
    delegator: Box<SDelegator>,
}

impl Delegator {
    fn new(p: i32, phrase: String) -> Self {
        let default_del = Delegate::new(Some(default_response));
        Delegator {
            delegator: Box::new(SDelegator {
                param: p,
                phrase,
                delegate: default_del,
            }),
        }
    }

    fn operation(&self, p1: i32, delroy: Option<&Delegate>) -> &'static str {
        let rtn = match delroy {
            Some(del) => match del.thing(p1) {
                Some(response) => response,
                None => self.delegator.delegate.thing(p1).unwrap_or(""),
            },
            None => self.delegator.delegate.thing(p1).unwrap_or(""),
        };
        println!("{}", self.delegator.phrase);
        rtn
    }
}

fn default_response(p1: i32) -> &'static str {
    "default implementation"
}

fn thing1(p1: i32) -> &'static str {
    println!("We're in thing1 with value {}", p1);
    "delegate implementation"
}

fn main() {
    let del1 = Delegate::new(Some(thing1));
    let del2 = Delegate::new(None);
    let the_delegator = Delegator::new(14, "A stellar vista, Baby.".to_string());

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