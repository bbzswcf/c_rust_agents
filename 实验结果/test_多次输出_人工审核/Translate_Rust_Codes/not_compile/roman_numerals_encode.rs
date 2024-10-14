use std::process;
use std::io::{self, Write};

fn roman(s: &mut [u8], mut n: u32) {
    if n == 0 {
        eprintln!("Roman numeral for zero requested.");
        process::exit(1);
    }

    macro_rules! digit {
        ($loop:tt, $num:expr, $c:expr) => {
            $loop {
                if n >= $num {
                    s.push($c);
                    n -= $num;
                } else {
                    break;
                }
            }
        };
    }

    macro_rules! digits {
        ($loop:tt, $num:expr, $c1:expr, $c2:expr) => {
            $loop {
                if n >= $num {
                    s.push($c1);
                    s.push($c2);
                    n -= $num;
                } else {
                    break;
                }
            }
        };
    }

    digit!(while, 1000, b'M');
    digits!(if, 900, b'C', b'M');
    digit!(if, 500, b'D');
    digits!(if, 400, b'C', b'D');
    digit!(while, 100, b'C');
    digits!(if, 90, b'X', b'C');
    digit!(if, 50, b'L');
    digits!(if, 40, b'X', b'L');
    digit!(while, 10, b'X');
    digits!(if, 9, b'I', b'X');
    digit!(if, 5, b'V');
    digits!(if, 4, b'I', b'V');
    digit!(while, 1, b'I');

    s.push(b'\0');
}

fn main() {
    let mut buffer = Vec::with_capacity(16);
    for i in 1..4000 {
        buffer.clear();
        roman(&mut buffer, i);
        println!("{:4}: {}", i, String::from_utf8_lossy(&buffer));
    }
}