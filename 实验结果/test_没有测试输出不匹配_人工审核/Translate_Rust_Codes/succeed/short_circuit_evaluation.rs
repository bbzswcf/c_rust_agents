fn a(in_: bool) -> bool {
    println!("I am a");
    in_
}

fn b(in_: bool) -> bool {
    println!("I am b");
    in_
}

macro_rules! test {
    ($X:expr, $Y:expr, $O:tt) => {
        {
            // Modified: Explicitly specify the type of `x` as `bool`
            // and use explicit logical operators `&&` and `||`
            let x: bool = match $O {
                "&&" => a($X) && b($Y),
                "||" => a($X) || b($Y),
                _ => panic!("Unsupported operator"),
            };
            println!("{:?} {} {:?} = {}", $X, stringify!($O), $Y, if x { "true" } else { "false" });
        }
    };
}

fn main() {
    // Modified: Initialize `x` with a default value to resolve type inference issues
    let mut x = false;

    test!(false, true, "&&"); // b is not evaluated
    test!(true, false, "||"); // b is not evaluated
    test!(true, false, "&&"); // b is evaluated
    test!(false, false, "||"); // b is evaluated
}