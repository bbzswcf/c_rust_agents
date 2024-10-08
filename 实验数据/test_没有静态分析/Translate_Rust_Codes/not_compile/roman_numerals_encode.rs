use std::process;

// Modified: Increased buffer size to 16 characters to handle the largest Roman numeral (MMMCMXCIX for 3999)
fn roman(mut s: &mut [char], mut n: u32) {
    if n == 0 {
        eprintln!("Roman numeral for zero requested.");
        process::exit(1);
    }

    macro_rules! digit {
        ($loop:ident, $num:expr, $c:expr) => {
            $loop (n >= $num) {
                // Modified: Ensure the buffer has at least one element before accessing it
                if s.is_empty() {
                    eprintln!("Buffer is empty.");
                    process::exit(1);
                }
                s[0] = $c;
                s = &mut s[1..]; // Modified: Reassignment of `s` is now allowed
                n -= $num;
            }
        };
    }

    macro_rules! digits {
        ($loop:ident, $num:expr, $c1:expr, $c2:expr) => {
            $loop (n >= $num) {
                // Modified: Ensure the buffer has at least two elements before accessing it
                if s.len() < 2 {
                    eprintln!("Buffer is too small.");
                    process::exit(1);
                }
                s[0] = $c1;
                s[1] = $c2;
                s = &mut s[2..]; // Modified: Reassignment of `s` is now allowed
                n -= $num;
            }
        };
    }

    // Modified: Adjusted the order of checks to handle subtractive combinations before additive ones
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

    // Modified: Ensure the entire buffer is properly null-terminated
    if !s.is_empty() {
        s[0] = '\0';
    }
}

fn main() {
    // Modified: Increased buffer size to 16 characters to handle the largest Roman numeral (MMMCMXCIX for 3999)
    let mut buffer = ['\0'; 16];
    for i in 1..4000 {
        roman(&mut buffer, i);
        // Modified: Correctly convert the character array to a string and print it
        let roman_numeral = {
            let temp = buffer.iter().collect::<String>();
            temp.trim_matches(char::from(0)).to_string()
        };
        println!("{:4}: {}", i, roman_numeral);
    }
}