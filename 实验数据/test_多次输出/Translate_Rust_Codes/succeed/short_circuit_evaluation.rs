fn a(in_: bool) -> bool {
    println!("I am a");
    in_
}

fn b(in_: bool) -> bool {
    println!("I am b");
    in_
}

macro_rules! test {
    ($x:expr, $y:expr, $op:tt) => {
        {
            let x = a($x) $op b($y);
            println!("{} {} {} = {}\n", stringify!($x), stringify!($op), stringify!($y), if x { "true" } else { "false" });
        }
    };
}

fn main() {
    test!(false, true, &&); // b is not evaluated
    test!(true, false, ||); // b is not evaluated
    test!(true, false, &&); // b is evaluated
    test!(false, false, ||); // b is evaluated
}