use std::sync::atomic::{AtomicI32, Ordering};

fn solve(n: i32, col: i32, hist: &mut [i32]) {
    if col == n {
        // Modified: Wrapped the usage of the mutable static variable `count` in an `unsafe` block
        // to ensure safe access.
        unsafe {
            println!("\nNo. {}\n-----", count.fetch_add(1, Ordering::SeqCst) + 1);
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

// Modified: Changed the type of `count` to `AtomicI32` to avoid data races.
static count: AtomicI32 = AtomicI32::new(0);

fn main() {
    let mut hist = [0; 8];
    solve(8, 0, &mut hist);
}