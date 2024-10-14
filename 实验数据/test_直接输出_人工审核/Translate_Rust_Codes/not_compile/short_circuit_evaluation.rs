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
            println!("{:?} {} {:?} = {}", $X, stringify!($O), $Y, if x { "true" } else { "false" });
        }
    };
}

fn main() {
    let mut x;

    test!(false, true, &&); // b is not evaluated
    test!(true, false, ||); // b is not evaluated
    test!(true, false, &&); // b is evaluated
    test!(false, false, ||); // b is evaluated
}