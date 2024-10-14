use std::process;

// Modified: Made the argument `s` mutable in the function signature
fn roman(mut s: &mut [char], mut n: u32) {
    if n == 0 {
        eprint!("Roman numeral for zero requested.");
        process::exit(1);
    }

    macro_rules! digit {
        ($loop:tt, $num:expr, $c:expr) => {
            $loop (n >= $num) {
                s[0] = $c;
                s = &mut s[1..]; // Fixed: Immutable argument assignment error
                n -= $num;
            }
        };
    }

    macro_rules! digits {
        ($loop:tt, $num:expr, $c1:expr, $c2:expr) => {
            $loop (n >= $num) {
                s[0] = $c1;
                s[1] = $c2;
                s = &mut s[2..]; // Fixed: Immutable argument assignment error
                n -= $num;
            }
        };
    }

    digit!(while, 1000, 'M');
    digits!(if, 900, 'C', 'M');
    digit!(if, 500, 'D');
    digits!(if, 400, 'C', 'D');
    digit!(while, 100, 'C');
    digits!(if, 90, 'X', 'C');
    digit!(if, 50, 'L');
    digits!(if, 40, 'X', 'L');
    digit!(while, 10, 'X');
    digits!(if, 9, 'I', 'X');
    digit!(if, 5, 'V');
    digits!(if, 4, 'I', 'V');
    digit!(while, 1, 'I');

    s[0] = '\0';
}

fn main() {
    let mut buffer = ['\0'; 16];
    for i in 1..4000 {
        roman(&mut buffer, i);
        println!("{:4}: {}", i, buffer.iter().collect::<String>());
    }
}