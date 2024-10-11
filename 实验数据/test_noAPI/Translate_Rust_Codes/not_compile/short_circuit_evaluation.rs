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
            // Modified: Use the logical operators directly within the expression
            let x: bool = match $O {
                "&&" => a($X) && b($Y),
                "||" => a($X) || b($Y),
                _ => panic!("Unsupported logical operator"),
            };
            println!("{} {} {} = {}\n", stringify!($X), $O, stringify!($Y), if x { "true" } else { "false" });
        }
    };
}

fn main() {
    let mut x;

    // Modified: Use the logical operators directly in the macro invocation
    test!(false, true, "&&"); // b is not evaluated
    test!(true, false, "||"); // b is not evaluated
    test!(true, false, "&&"); // b is evaluated
    test!(false, false, "||"); // b is evaluated

    // Modified: Provide a type annotation to specify the type parameter for `Vec`
    let x: Vec<i32> = Vec::new();

    // Modified: Specify the generic type parameter for `Foo` to resolve the ambiguity
    struct Foo<T> {
        value: T,
    }

    impl<T> Foo<T> {
        fn bar() -> T {
            unimplemented!()
        }
    }

    let number = Foo::<i32>::bar();
}