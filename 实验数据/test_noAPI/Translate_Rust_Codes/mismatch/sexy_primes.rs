// Removed unused import
// use std::mem;

const TRUE: u8 = 1;
const FALSE: u8 = 0;

fn sieve(c: &mut [u8], limit: usize) {
    let mut p = 3;
    let mut p2;
    c[0] = TRUE;
    c[1] = TRUE;
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
            if c[p] == 0 {
                break;
            }
        }
    }
}

fn print_helper(cat: &str, len: usize, lim: usize, n: usize) {
    let sp = if cat == "unsexy primes" { "" } else { "sexy prime " };
    let verb = if len == 1 { "is" } else { "are" };
    // Removed unused argument `len` from the format string
    println!("Number of {}less than {} = {}", sp, cat, lim);
    println!("The last {} {}:", n, verb);
}

fn print_array(a: &[i32]) {
    print!("[");
    for &item in a {
        print!("{} ", item);
    }
    // Used raw string literal to avoid interpreting `\b` as an escape sequence
    print!(r"\b]");
}

fn main() {
    let lim = 1_000_035;
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
    let mut sv = vec![0; lim - 1];

    sieve(&mut sv, lim);

    for i in (3..lim).step_by(2) {
        if i > 5 && i < lim - 6 && sv[i] == 0 && sv[i - 6] == 1 && sv[i + 6] == 1 {
            unsexy += 1;
            continue;
        }
        if i < lim - 6 && sv[i] == 0 && sv[i + 6] == 0 {
            pairs += 1;
        } else {
            continue;
        }

        if i < lim - 12 && sv[i + 12] == 0 {
            trips += 1;
        } else {
            continue;
        }

        if i < lim - 18 && sv[i + 18] == 0 {
            quads += 1;
        } else {
            continue;
        }

        if i < lim - 24 && sv[i + 24] == 0 {
            quins += 1;
        }
    }

    if pairs < lpr {
        pairs = lpr;
    }
    if trips < ltr {
        trips = ltr;
    }
    if quads < lqd {
        quads = lqd;
    }
    if quins < lqn {
        quins = lqn;
    }
    if unsexy < lun {
        unsexy = lun;
    }

    for i in (3..lim).step_by(2) {
        if i > 5 && i < lim - 6 && sv[i] == 0 && sv[i - 6] == 1 && sv[i + 6] == 1 {
            un += 1;
            if un > unsexy - lun {
                last_un[un + lun - 1 - unsexy] = i as i32;
            }
            continue;
        }
        if i < lim - 6 && sv[i] == 0 && sv[i + 6] == 0 {
            pr += 1;
            if pr > pairs - lpr {
                let ix = pr + lpr - 1 - pairs;
                last_pr[ix][0] = i as i32;
                last_pr[ix][1] = (i + 6) as i32;
            }
        } else {
            continue;
        }

        if i < lim - 12 && sv[i + 12] == 0 {
            tr += 1;
            if tr > trips - ltr {
                let ix = tr + ltr - 1 - trips;
                last_tr[ix][0] = i as i32;
                last_tr[ix][1] = (i + 6) as i32;
                last_tr[ix][2] = (i + 12) as i32;
            }
        } else {
            continue;
        }

        if i < lim - 18 && sv[i + 18] == 0 {
            qd += 1;
            if qd > quads - lqd {
                let ix = qd + lqd - 1 - quads;
                last_qd[ix][0] = i as i32;
                last_qd[ix][1] = (i + 6) as i32;
                last_qd[ix][2] = (i + 12) as i32;
                last_qd[ix][3] = (i + 18) as i32;
            }
        } else {
            continue;
        }

        if i < lim - 24 && sv[i + 24] == 0 {
            qn += 1;
            if qn > quins - lqn {
                let ix = qn + lqn - 1 - quins;
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
        // Used raw string literal to avoid interpreting `\b` as an escape sequence
        print!(r"\b] ");
    }
    // Used raw string literal to avoid interpreting `\b` as an escape sequence
    print!(r"\b]\n\n");

    print_helper("triplets", trips, lim, ltr);
    print!("  [");
    for i in 0..ltr {
        print_array(&last_tr[i]);
        // Used raw string literal to avoid interpreting `\b` as an escape sequence
        print!(r"\b] ");
    }
    // Used raw string literal to avoid interpreting `\b` as an escape sequence
    print!(r"\b]\n\n");

    print_helper("quadruplets", quads, lim, lqd);
    print!("  [");
    for i in 0..lqd {
        print_array(&last_qd[i]);
        // Used raw string literal to avoid interpreting `\b` as an escape sequence
        print!(r"\b] ");
    }
    // Used raw string literal to avoid interpreting `\b` as an escape sequence
    print!(r"\b]\n\n");

    print_helper("quintuplets", quins, lim, lqn);
    print!("  [");
    for i in 0..lqn {
        print_array(&last_qn[i]);
        // Used raw string literal to avoid interpreting `\b` as an escape sequence
        print!(r"\b] ");
    }
    // Used raw string literal to avoid interpreting `\b` as an escape sequence
    print!(r"\b]\n\n");

    print_helper("unsexy primes", unsexy, lim, lun);
    print!("  [");
    print_array(&last_un);
    // Used raw string literal to avoid interpreting `\b` as an escape sequence
    print!(r"\b]\n");
}