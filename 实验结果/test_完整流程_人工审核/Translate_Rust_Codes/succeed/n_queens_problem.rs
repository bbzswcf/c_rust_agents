use std::sync::atomic::{AtomicI32, Ordering};

fn solve(n: i32, col: i32, hist: &mut [i32]) {
    if col == n {
        println!("\nNo. {}\n-----", COUNT.fetch_add(1, Ordering::SeqCst) + 1);
        for i in 0..n {
            for j in 0..n {
                print!("{}", if j == hist[i as usize] { 'Q' } else if (i + j) & 1 == 1 { ' ' } else { '.' });
            }
            println!();
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

static COUNT: AtomicI32 = AtomicI32::new(0); // Modified: Replaced `static mut count: i32 = 0;` with `AtomicI32` for safe concurrent access

fn main() {
    let mut hist = [0; 8];
    solve(8, 0, &mut hist);
}