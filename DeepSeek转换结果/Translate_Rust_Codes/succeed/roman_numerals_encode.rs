fn roman(s: &mut Vec<char>, mut n: u32) {
    if n == 0 {
        eprint!("Error: Roman numeral for zero requested.");
        std::process::exit(1);
    }

    // 修改: 使用 `while` 循环代替 `if` 和 `while` 混合使用
    macro_rules! digit {
        ($num:expr, $c:expr) => {
            while n >= $num {
                s.push($c);
                n -= $num;
            }
        };
    }

    // 修改: 使用 `while` 循环代替 `if` 和 `while` 混合使用
    macro_rules! digits {
        ($num:expr, $c1:expr, $c2:expr) => {
            while n >= $num {
                s.push($c1);
                s.push($c2);
                n -= $num;
            }
        };
    }

    digit!(1000, 'M');
    digits!(900, 'C', 'M');
    digit!(500, 'D');
    digits!(400, 'C', 'D');
    digit!(100, 'C');
    digits!(90, 'X', 'C');
    digit!(50, 'L');
    digits!(40, 'X', 'L');
    digit!(10, 'X');
    digits!(9, 'I', 'X');
    digit!(5, 'V');
    digits!(4, 'I', 'V');
    digit!(1, 'I');

    s.push('\0');
}

fn main() {
    let mut buffer = Vec::new();
    for i in 1..4000 {
        roman(&mut buffer, i);
        print!("{:4}: ", i);
        for &ch in buffer.iter() {
            if ch == '\0' {
                break;
            }
            print!("{}", ch);
        }
        println!();
        buffer.clear();
    }
    std::process::exit(0);
}