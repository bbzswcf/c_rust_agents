use std::io::{self, Write};

static mut COUNT: i32 = 0;

fn solve(n: i32, col: i32, hist: &mut [i32]) {
    if col == n {
        unsafe {
            println!("\nNo. {}\n-----", COUNT + 1);
            COUNT += 1;
        }
        for i in 0..n {
            for j in 0..n {
                if j == hist[i as usize] {
                    print!("Q");
                } else {
                    print!("{}", if (i + j) & 1 == 1 { ' ' } else { '.' });
                }
            }
            println!();
        }
        return;
    }

    macro_rules! attack {
        ($i:expr, $j:expr) => {
            hist[$j as usize] == $i || (hist[$j as usize] - $i).abs() == col - $j
        };
    }

    for i in 0..n {
        let mut j = 0;
        while j < col && !attack!(i, j) {
            j += 1;
        }
        if j < col {
            continue;
        }

        hist[col as usize] = i;
        solve(n, col + 1, hist);
    }
}

fn main() -> io::Result<()> {
    let mut hist = [0; 8];
    solve(8, 0, &mut hist);
    Ok(())
}