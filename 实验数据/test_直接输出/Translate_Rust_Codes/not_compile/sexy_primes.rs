use std::cmp::Ordering;

type Bool = u8;

const TRUE: Bool = 1;
const FALSE: Bool = 0;

fn sieve(c: &mut [Bool], limit: usize) {
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
    print!("\b]");
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
    let mut last_pr = vec![[0; 2]; lpr];
    let mut last_tr = vec![[0; 3]; ltr];
    let mut last_qd = vec![[0; 4]; lqd];
    let mut last_qn = vec![[0; 5]; lqn];
    let mut last_un = vec![0; lun];
    let mut sv = vec![FALSE; lim - 1];

    sieve(&mut sv, lim);

    for i in (3..lim).step_by(2) {
        if i > 5 && i < lim - 6 && sv[i] == FALSE && sv[i - 6] == TRUE && sv[i + 6] == TRUE {
            unsexy += 1;
            continue;
        }
        if i < lim - 6 && sv[i] == FALSE && sv[i + 6] == FALSE {
            pairs += 1;
        } else {
            continue;
        }

        if i < lim - 12 && sv[i + 12] == FALSE {
            trips += 1;
        } else {
            continue;
        }

        if i < lim - 18 && sv[i + 18] == FALSE {
            quads += 1;
        } else {
            continue;
        }

        if i < lim - 24 && sv[i + 24] == FALSE {
            quins += 1;
        }
    }

    if pairs < lpr {
        last_pr.resize(pairs, [0; 2]);
    }
    if trips < ltr {
        last_tr.resize(trips, [0; 3]);
    }
    if quads < lqd {
        last_qd.resize(quads, [0; 4]);
    }
    if quins < lqn {
        last_qn.resize(quins, [0; 5]);
    }
    if unsexy < lun {
        last_un.resize(unsexy, 0);
    }

    for i in (3..lim).step_by(2) {
        if i > 5 && i < lim - 6 && sv[i] == FALSE && sv[i - 6] == TRUE && sv[i + 6] == TRUE {
            un += 1;
            if un > unsexy - lun {
                last_un[un + lun - 1 - unsexy] = i as i32;
            }
            continue;
        }
        if i < lim - 6 && sv[i] == FALSE && sv[i + 6] == FALSE {
            pr += 1;
            if pr > pairs - lpr {
                let ix = pr + lpr - 1 - pairs;
                last_pr[ix][0] = i as i32;
                last_pr[ix][1] = (i + 6) as i32;
            }
        } else {
            continue;
        }

        if i < lim - 12 && sv[i + 12] == FALSE {
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

        if i < lim - 18 && sv[i + 18] == FALSE {
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

        if i < lim - 24 && sv[i + 24] == FALSE {
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

    print_helper("unsexy primes", unsexy, lim, lun);
    print!("  [");
    print_array(&last_un);
    print!("\b]\n");
}