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
            // Annotate the type of `x` to help the compiler infer it
            let x: bool = match $O {
                "&&" => {
                    let a_result = a($X);
                    if a_result {
                        b($Y)
                    } else {
                        false
                    }
                },
                "||" => {
                    let a_result = a($X);
                    if a_result {
                        true
                    } else {
                        b($Y)
                    }
                },
                _ => panic!("Unsupported operator"),
            };
            println!("{} {} {} = {}\n", stringify!($X), $O, stringify!($Y), if x { "true" } else { "false" });
        }
    };
}

fn main() {
    // Removed: Unused variable _x

    test!(false, true, "&&"); // b is not evaluated
    test!(true, false, "||"); // b is not evaluated
    test!(true, false, "&&"); // b is evaluated
    test!(false, false, "||"); // b is evaluated
}