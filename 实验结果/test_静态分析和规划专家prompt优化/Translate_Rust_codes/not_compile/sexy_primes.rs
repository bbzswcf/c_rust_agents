use std::alloc::{alloc, Layout};
const TRUE: u8 = 1;
const FALSE: u8 = 0;
type Bool = u8;

fn sieve(c: &mut [Bool], limit: usize) {
    let mut i;
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
        i = p2;
        while i < limit {
            c[i] = TRUE;
            i += 2 * p;
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
    println!("Number of {}less than {} = {}", sp, cat, len);
    println!("The last {} {}:", n, verb);
}

fn print_array(a: &[i32]) {
    print!("[");
    for &item in a {
        print!("{} ", item);
    }
    print!("\x08]");
}

fn main() {
    let lim = 1000035;
    let mut pairs: usize = 0;
    let mut trips: usize = 0;
    let mut quads: usize = 0;
    let mut quins: usize = 0;
    let mut unsexy: usize = 2;
    let mut pr = 0;
    let mut tr = 0;
    let mut qd = 0;
    let mut qn = 0;
    let mut un = 2;
    const lpr: usize = 5;
    const ltr: usize = 5;
    const lqd: usize = 5;
    const lqn: usize = 5;
    const lun: usize = 10;
    let mut last_pr = vec![[0; 2]; 5];
    let mut last_tr = vec![[0; 3]; 5];
    let mut last_qd = vec![[0; 4]; 5];
    let mut last_qn = vec![[0; 5]; 5];
    let mut last_un = vec![0; 10];
    let layout = Layout::array::<Bool>(lim - 1).unwrap();
    let ptr = unsafe { alloc(layout) };
    let sv = unsafe { std::slice::from_raw_parts_mut(ptr as *mut Bool, lim - 1) };
    sieve(sv, lim);

    // get the counts first
    let mut i = 3;
    while i < lim {
        if i > 5 && i < lim - 6 && sv[i] == FALSE && sv[i - 6] == FALSE && sv[i + 6] == FALSE {
            unsexy += 1;
            continue;
        }
        if i < lim - 6 && sv[i] == FALSE && sv[i + 6] == TRUE {
            pairs += 1;
        } else {
            i += 2;
            continue;
        }
        if i < lim - 12 && sv[i + 12] == TRUE {
            trips += 1;
        } else {
            i += 2;
            continue;
        }
        if i < lim - 18 && sv[i + 18] == TRUE {
            quads += 1;
        } else {
            i += 2;
            continue;
        }
        if i < lim - 24 && sv[i + 24] == TRUE {
            quins += 1;
        }
        i += 2;
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

    // now get the last 'x' for each category
    i = 3;
    while i < lim {
        if i > 5 && i < lim - 6 && sv[i] == FALSE && sv[i - 6] == FALSE && sv[i + 6] == FALSE {
            un += 1;
            if un > unsexy - lun {
                last_un[un + lun - 1 - unsexy] = i as i32;
            }
            i += 2;
            continue;
        }
        if i < lim - 6 && sv[i] == FALSE && sv[i + 6] == TRUE {
            pr += 1;
            if pr > pairs - lpr {
                let ix = pr + lpr - 1 - pairs;
                last_pr[ix][0] = i as i32;
                last_pr[ix][1] = (i + 6) as i32;
            }
        } else {
            i += 2;
            continue;
        }
        if i < lim - 12 && sv[i + 12] == TRUE {
            tr += 1;
            if tr > trips - ltr {
                let ix = tr + ltr - 1 - trips;
                last_tr[ix][0] = i as i32;
                last_tr[ix][1] = (i + 6) as i32;
                last_tr[ix][2] = (i + 12) as i32;
            }
        } else {
            i += 2;
            continue;
        }
        if i < lim - 18 && sv[i + 18] == TRUE {
            qd += 1;
            if qd > quads - lqd {
                let ix = qd + lqd - 1 - quads;
                last_qd[ix][0] = i as i32;
                last_qd[ix][1] = (i + 6) as i32;
                last_qd[ix][2] = (i + 12) as i32;
                last_qd[ix][3] = (i + 18) as i32;
            }
        } else {
            i += 2;
            continue;
        }
        if i < lim - 24 && sv[i + 24] == TRUE {
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
        i += 2;
    }

    print_helper("pairs", pairs, lim, lpr);
    print!("  [");
    for i in 0..lpr {
        print_array(&last_pr[i]);
        print!("\x08] ");
    }
    print!("\x08]\n\n");

    print_helper("triplets", trips, lim, ltr);
    print!("  [");
    for i in 0..ltr {
        print_array(&last_tr[i]);
        print!("\x08] ");
    }
    print!("\x08]\n\n");

    print_helper("quadruplets", quads, lim, lqd);
    print!("  [");
    for i in 0..lqd {
        print_array(&last_qd[i]);
        print!("\x08] ");
    }
    print!("\x08]\n\n");

    print_helper("quintuplets", quins, lim, lqn);
    print!("  [");
    for i in 0..lqn {
        print_array(&last_qn[i]);
        print!("\x08] ");
    }
    print!("\x08]\n\n");

    print_helper("unsexy primes", unsexy, lim, lun);
    print!("  [");
    print_array(&last_un);
    print!("\x08]\n");
}