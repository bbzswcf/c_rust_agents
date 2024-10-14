use std::io;

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
            let x = a($X) $O b($Y);
            println!("{} {} {} = {}", stringify!($X), stringify!($O), stringify!($Y), if x { "true" } else { "false" });
        }
    };
}

fn main() -> io::Result<()> {
    // Modified: Provided an explicit type for the variable `x` as `bool`
    let mut x: bool;

    test!(false, true, &&); // b is not evaluated
    test!(true, false, ||); // b is not evaluated
    test!(true, false, &&); // b is evaluated
    test!(false, false, ||); // b is evaluated

    Ok(())
}