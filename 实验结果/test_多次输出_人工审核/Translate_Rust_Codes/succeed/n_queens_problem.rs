fn solve(n: usize, col: usize, hist: &mut [usize], count: &mut usize) {
    if col == n {
        *count += 1;
        println!("\nNo. {}\n-----", count);
        for i in 0..n {
            for j in 0..n {
                print!("{}", if j == hist[i] { 'Q' } else if (i + j) & 1 == 1 { ' ' } else { '.' });
            }
            println!();
        }
        return;
    }

    macro_rules! attack {
        ($i:expr, $j:expr) => {
            hist[$j] == $i || hist[$j].abs_diff($i) == col - $j
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

        hist[col] = i;
        solve(n, col + 1, hist, count);
    }
}

fn main() {
    let mut hist = vec![0; 8];
    let mut count = 0;
    solve(8, 0, &mut hist, &mut count);
}