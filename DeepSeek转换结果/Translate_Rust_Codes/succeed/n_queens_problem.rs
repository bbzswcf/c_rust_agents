fn solve(n: i32, col: i32, hist: &mut [i32; 8]) {
    if col == n {
        unsafe {
            print!("\nNo. {}\n-----\n", { count += 1; count });
        }
        for i in 0..n {
            for j in 0..n {
                print!("{}", if j == hist[i as usize] { 'Q' } else { if (i + j) & 1 == 1 { ' ' } else { '.' } });
            }
            print!("\n");
        }
        return;
    }

    macro_rules! attack {
        ($i:expr, $j:expr) => {
            hist[$j as usize] == $i || i32::abs(hist[$j as usize] - $i) == col - $j
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

static mut count: i32 = 0;

fn main() {
    let mut hist: [i32; 8] = [0; 8];
    solve(8, 0, &mut hist);
}