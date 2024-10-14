```rust
use std::cmp::min;
use std::fmt::Write;
use std::mem::MaybeUninit;

fn sieve(c: &mut [bool], limit: usize) {
    let mut p = 3;
    // TRUE denotes composite, FALSE denotes prime.
    c[0] = true;
    c[1] = true;
    // no need to bother with even numbers over 2 for this task
    loop {
        let p2 = p * p;
        if p2 >= limit {
            break;
        }
        for i in (p2..limit).step_by(2 * p) {
            c[i] = true;
        }
        loop {
            p += 2;
            if !c[p] {
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
    let mut output = String::new();
    write!(output, "[").unwrap();
    for &num in a {
        write!(output, "{} ", num).unwrap();
    }
    write!(output, "\b]").unwrap();
    print!("{}", output);
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
    let mut last_pr: [[i32; 2]; 5] = unsafe { MaybeUninit::uninit().assume_init() };
    let mut last_tr: [[i32; 3]; 5] = unsafe { MaybeUninit::uninit().assume_init() };
    let mut last_qd: [[i32; 4]; 5] = unsafe { MaybeUninit::uninit().assume_init() };
    let mut last_qn: [[i32; 5]; 5] = unsafe { MaybeUninit::uninit().assume_init() };
    let mut last_un: [i32; 10] = unsafe { MaybeUninit::uninit().assume_init() };
    let mut sv = vec![false; lim - 1];

    sieve(&mut sv, lim);

    // get the counts first
    for i in (3..lim).step_by(2) {
        if i > 5 && i < lim - 6 && !sv[i] && sv[i - 6] && sv[i + 6] {
            unsexy += 1;
            continue;
        }
        if i < lim - 6 && !sv[i] && !sv[i + 6] {
            pairs += 1;
        } else {
            continue;
        }

        if i < lim - 12 && !sv[i + 12] {
            trips += 1;
        } else {
            continue;
        }

        if i < lim - 18 && !sv[i + 18] {
            quads += 1;
        } else {
            continue;
        }

        if i < lim - 24 && !sv[i + 24] {
            quins += 1;
        }
    }
    let lpr = min(lpr, pairs);
    let ltr = min(ltr, trips);
    let lqd = min(lqd, quads);
    let lqn = min(lqn, quins);
    let lun = min(lun, unsexy);

    // now get the last 'x' for each category
    for i in (3..lim).step_by(2) {
        if i > 5 && i < lim - 6 && !sv[i] && sv[i - 6] && sv[i + 6] {
            un += 1;
            if un > unsexy - lun {
                last_un[un + lun - 1 - unsexy] = i as i32;
            }
            continue;
        }
        if i < lim - 6 && !sv[i] && !sv[i + 6] {
            pr += 1;
            if pr > pairs - lpr {
                let ix = pr + lpr - 1 - pairs;
                last_pr[ix][0] = i as i32;
                last_pr[ix][1] = (i + 6) as i32;
            }
        } else {
            continue;
        }

        if i < lim - 12 && !sv[i + 12] {
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

        if i < lim - 18 && !sv[i + 18] {
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

        if i < lim - 24 && !sv[i + 24] {
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
        print!("\b] ");
    }
    print!("\b]\n\n");

    print_helper("triplets", trips, lim, ltr);
    print!("  [");
    for i in 0..ltr {
        print_array(&last_tr[i]);
        print!("\b] ");
    }
    print!("\b]\n\n");

    print_helper("quadruplets", quads, lim, lqd);
    print!("  [");
    for i in 0..lqd {
        print_array(&last_qd[i]);
        print!("\b] ");
    }
    print!("\b]\n\n");

    print_helper("quintuplets", quins, lim, lqn);
    print!("  [");
    for i in 0..lqn {
        print_array(&last_qn[i]);
        print!("\b] ");
    }
    print!("\b]\n\n");

   