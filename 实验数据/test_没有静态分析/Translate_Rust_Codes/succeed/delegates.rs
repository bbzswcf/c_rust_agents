use std::boxed::Box;
use std::sync::LazyLock;

// Modified: Changed Responder type to Option<fn(i32) -> &'static str>
type Responder = Option<fn(i32) -> &'static str>;

#[derive(Clone)]
struct SDelegate {
    operation: Responder,
}

type Delegate = Box<SDelegate>;

fn new_delegate(rspndr: Responder) -> Delegate {
    Box::new(SDelegate { operation: rspndr })
}

fn delegate_thing(dl: &Delegate, p1: i32) -> Option<&'static str> {
    // Modified: Check if the function pointer is None before calling it
    if let Some(op) = dl.operation {
        Some(op(p1))
    } else {
        None
    }
}

struct SDelegator {
    param: i32,
    phrase: String,
    delegate: Delegate,
}

type Delegator = Box<SDelegator>;

fn default_response(p1: i32) -> &'static str {
    "default implementation"
}

// Modified: Use LazyLock to lazily initialize the static variable
static DEFAULT_DEL: LazyLock<Delegate> = LazyLock::new(|| Box::new(SDelegate { operation: Some(default_response) }));

fn new_delegator(p: i32, phrase: &str) -> Delegator {
    Box::new(SDelegator {
        param: p,
        phrase: phrase.to_string(),
        delegate: DEFAULT_DEL.clone(), // Modified: Clone the default delegate
    })
}

fn delegator_operation(the_delegator: &Delegator, p1: i32, delroy: Option<&Delegate>) -> &'static str {
    let rtn = if let Some(delroy) = delroy {
        if let Some(result) = delegate_thing(delroy, p1) {
            result
        } else {
            delegate_thing(&the_delegator.delegate, p1).unwrap_or_default()
        }
    } else {
        delegate_thing(&the_delegator.delegate, p1).unwrap_or_default()
    };

    println!("{}", the_delegator.phrase);
    rtn
}

fn thing1(p1: i32) -> &'static str {
    println!("We're in thing1 with value {}", p1);
    "delegate implementation"
}

fn main() {
    let del1 = new_delegate(Some(thing1));
    // Modified: Pass a valid function pointer instead of a null pointer
    let del2 = new_delegate(Some(default_response));
    let the_delegator = new_delegator(14, "A stellar vista, Baby.");

    println!("Delegator returns {}\n", delegator_operation(&the_delegator, 3, None));
    println!("Delegator returns {}\n", delegator_operation(&the_delegator, 3, Some(&del1)));
    println!("Delegator returns {}\n", delegator_operation(&the_delegator, 3, Some(&del2)));
}