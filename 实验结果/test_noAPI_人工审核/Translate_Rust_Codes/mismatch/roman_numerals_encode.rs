use std::io::{self, Write};

// Modified: Made `s` mutable and `n` mutable in the function signature
fn roman(mut s: &mut [u8], mut n: u32) {
    if n == 0 {
        writeln!(io::stderr(), "Roman numeral for zero requested.").unwrap();
        std::process::exit(1);
    }

    macro_rules! digit {
        ($loop:tt, $num:expr, $c:expr) => {
            $loop (n >= $num) {
                *s.get_mut(0).unwrap() = $c;
                s = &mut s[1..]; // Modified: `s` is now mutable
                n -= $num; // Modified: `n` is now mutable
            }
        };
    }

    macro_rules! digits {
        ($loop:tt, $num:expr, $c1:expr, $c2:expr) => {
            $loop (n >= $num) {
                *s.get_mut(0).unwrap() = $c1;
                *s.get_mut(1).unwrap() = $c2;
                s = &mut s[2..]; // Modified: `s` is now mutable
                n -= $num; // Modified: `n` is now mutable
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

    *s.get_mut(0).unwrap() = b'\0';
}

fn main() -> io::Result<()> {
    let mut buffer = [0u8; 16];
    for i in 1..4000 {
        roman(&mut buffer, i);
        println!("{:4}: {}", i, std::str::from_utf8(&buffer).unwrap());
    }
    Ok(())
}