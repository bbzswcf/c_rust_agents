use std::cmp::PartialEq;
use std::mem;

const TRUE: u8 = 1;
const FALSE: u8 = 0;

type Bool = u8;

fn sieve(c: &mut [Bool], limit: usize) {
    let mut p = 3;
    let mut p2;
    // TRUE denotes composite, FALSE denotes prime.
    c[0] = TRUE;
    c[1] = TRUE;
    // no need to bother with even numbers over 2 for this task
    loop {
        p2 = p * p;
        if p2 >= limit {
            break;
        }
        for i in (p2..limit).step_by(2 * p) {
            c[i] = TRUE;
        }
        loop {
            p += 2;
            if c[p] == FALSE {
                break;
            }
        }
    }
}

fn print_helper(cat: &str, len: usize, lim: usize, n: usize) {
    let sp = if cat == "unsexy primes" { "" } else { "sexy prime " };
    let verb = if len == 1 { "is" } else { "are" };
    println!("Number of {}{} less than {} = {}", sp, cat, lim, len);
    println!("The last {} {}:", n, verb);
}

fn print_array(a: &[i32]) {
    print!("[");
    for &item in a {
        print!("{} ", item);
    }
    print!("\x08]"); // Modified: Corrected raw string literal usage
}

fn main() {
    let lim = 1000035;
    let mut pairs = 0;
    let mut trips = 0;
    let mut quads = 0;
    let mut quins = 0;
    let mut unsexy = 2;
    let mut pr = 0;
    let mut tr = 0;
    let mut qd = 0;
    let mut qn = 0;
    let mut un = 2;
    let lpr = 5;
    let ltr = 5;
    let lqd = 5;
    let lqn = 5;
    let lun = 10;
    let mut last_pr = [[0; 2]; 5];
    let mut last_tr = [[0; 3]; 5];
    let mut last_qd = [[0; 4]; 5];
    let mut last_qn = [[0; 5]; 5];
    let mut last_un = [0; 10];
    let mut sv = vec![FALSE; lim - 1]; // all FALSE by default

    sieve(&mut sv, lim);

    // get the counts first
    for i in (3..lim).step_by(2) {
        if i > 5 && i < lim - 6 && sv[i] == FALSE && sv[i - 6] == TRUE && sv[i + 6] == TRUE {
            unsexy += 1;
        }
        if i < lim - 6 && sv[i] == FALSE && sv[i + 6] == FALSE {
            pairs += 1;
        }

        if i < lim - 12 && sv[i + 12] == FALSE {
            trips += 1;
        }

        if i < lim - 18 && sv[i + 18] == FALSE {
            quads += 1;
        }

        if i < lim - 24 && sv[i + 24] == FALSE {
            quins += 1;
        }
    }
    if pairs < lpr {
        pr = pairs;
    }
    if trips < ltr {
        tr = trips;
    }
    if quads < lqd {
        qd = quads;
    }
    if quins < lqn {
        qn = quins;
    }
    if unsexy < lun {
        un = unsexy;
    }

    // now get the last 'x' for each category
    for i in (3..lim).step_by(2) {
        if i > 5 && i < lim - 6 && sv[i] == FALSE && sv[i - 6] == TRUE && sv[i + 6] == TRUE {
            un += 1;
            if un > unsexy - lun {
                let index = un + lun - 1 - unsexy;
                // Removed: Unnecessary check for non-negative index
                last_un[index] = i as i32;
            }
        }
        if i < lim - 6 && sv[i] == FALSE && sv[i + 6] == FALSE {
            pr += 1;
            if pr > pairs - lpr {
                let ix = pr + lpr - 1 - pairs;
                // Removed: Unnecessary bounds check
                last_pr[ix][0] = i as i32;
                last_pr[ix][1] = (i + 6) as i32;
            }
        }

        if i < lim - 12 && sv[i + 12] == FALSE {
            tr += 1;
            if tr > trips - ltr {
                let ix = tr + ltr - 1 - trips;
                // Removed: Unnecessary bounds check
                last_tr[ix][0] = i as i32;
                last_tr[ix][1] = (i + 6) as i32;
                last_tr[ix][2] = (i + 12) as i32;
            }
        }

        if i < lim - 18 && sv[i + 18] == FALSE {
            qd += 1;
            if qd > quads - lqd {
                let ix = qd + lqd - 1 - quads;
                // Removed: Unnecessary bounds check
                last_qd[ix][0] = i as i32;
                last_qd[ix][1] = (i + 6) as i32;
                last_qd[ix][2] = (i + 12) as i32;
                last_qd[ix][3] = (i + 18) as i32;
            }
        }

        if i < lim - 24 && sv[i + 24] == FALSE {
            qn += 1;
            if qn > quins - lqn {
                let ix = qn + lqn - 1 - quins;
                // Removed: Unnecessary bounds check
                last_qn[ix][0] = i as i32;
                last_qn[ix][1] = (i + 6) as i32;
                last_qn[ix][2] = (i + 12) as i32;
                last_qn[ix][3] = (i + 18) as i32;
                last_qn[ix][4] = (i + 24) as i32;
            }
        }
    }

    print_helper("pairs", pairs, lim, lpr);
    print!("  [");
    for i in 0..lpr {
        print_array(&last_pr[i]);
        print!("\x08] "); // Modified: Corrected raw string literal usage
    }
    print!("\x08]\n\n"); // Modified: Corrected raw string literal usage

    print_helper("triplets", trips, lim, ltr);
    print!("  [");
    for i in 0..ltr {
        print_array(&last_tr[i]);
        print!("\x08] "); // Modified: Corrected raw string literal usage
    }
    print!("\x08]\n\n"); // Modified: Corrected raw string literal usage

    print_helper("quadruplets", quads, lim, lqd);
    print!("  [");
    for i in 0..lqd {
        print_array(&last_qd[i]);
        print!("\x08] "); // Modified: Corrected raw string literal usage
    }
    print!("\x08]\n\n"); // Modified: Corrected raw string literal usage

    print_helper("quintuplets", quins, lim, lqn);
    print!("  [");
    for i in 0..lqn {
        print_array(&last_qn[i]);
        print!("\x08] "); // Modified: Corrected raw string literal usage
    }
    print!("\x08]\n\n"); // Modified: Corrected raw string literal usage

    print_helper("unsexy primes", unsexy, lim, lun);
    print!("  [");
    print_array(&last_un);
    print!("\x08]\n"); // Modified: Corrected raw string literal usage
}