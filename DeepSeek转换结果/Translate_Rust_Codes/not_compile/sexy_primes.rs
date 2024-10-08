fn sieve(c: &mut Vec<bool>, limit: i32) {
    let mut p = 3;
    let mut p2;
    c[0] = true;
    c[1] = true;
    loop {
        p2 = p * p;
        //  p2  limit
        if p2 > limit {
            break;
        }
        for i in (p2..limit).step_by(2 * p as usize) {
            //  i  c 
            if i < c.len() as i32 {
                c[i as usize] = true;
            }
        }
        loop {
            p += 2;
            //  p  c 
            if p >= c.len() as i32 || !c[p as usize] {
                break;
            }
        }
    }
}

fn print_helper(cat: &str, len: i32, lim: i32, n: i32) {
    let sp = if cat.cmp("unsexy primes") != std::cmp::Ordering::Equal { "sexy prime " } else { "" };
    let verb = if len == 1 { "is" } else { "are" };
    print!("Number of {}less than {} = {}\n", sp, lim, len);
    print!("The last {} {}: \n", n, verb);
}

// :  print_array  &[i32] 
fn print_array(a: &[i32]) {
    print!("[");
    if !a.is_empty() {
        for i in a {
            print!("{} ", i);
        }
        // :  \b  \x08 
        print!("\x08");
    }
    print!("]");
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
    let mut last_pr = vec![[0; 2]; lpr as usize];
    let mut last_tr = vec![[0; 3]; ltr as usize];
    let mut last_qd = vec![[0; 4]; lqd as usize];
    let mut last_qn = vec![[0; 5]; lqn as usize];
    let mut last_un = vec![0; lun as usize];
    let mut sv = vec![false; (lim - 1) as usize];

    sieve(&mut sv, lim);

    for i in (3..lim).step_by(2) {
        if i > 5 && i < lim - 6 && !sv[i as usize] && sv[(i - 6) as usize] && sv[(i + 6) as usize] {
            unsexy += 1;
            continue;
        }
        if i < lim - 6 && !sv[i as usize] && !sv[(i + 6) as usize] {
            pairs += 1;
        } else {
            continue;
        }

        if i < lim - 12 && !sv[(i + 12) as usize] {
            trips += 1;
        } else {
            continue;
        }

        if i < lim - 18 && !sv[(i + 18) as usize] {
            quads += 1;
        } else {
            continue;
        }

        if i < lim - 24 && !sv[(i + 24) as usize] {
            quins += 1;
        }
    }

    if pairs < lpr {
        last_pr.resize(pairs as usize, [0; 2]);
    }
    if trips < ltr {
        last_tr.resize(trips as usize, [0; 3]);
    }
    if quads < lqd {
        last_qd.resize(quads as usize, [0; 4]);
    }
    if quins < lqn {
        last_qn.resize(quins as usize, [0; 5]);
    }
    if unsexy < lun {
        last_un.resize(unsexy as usize, 0);
    }

    for i in (3..lim).step_by(2) {
        if i > 5 && i < lim - 6 && !sv[i as usize] && sv[(i - 6) as usize] && sv[(i + 6) as usize] {
            un += 1;
            //  un + lun - 1 - unsexy 
            if un + lun - 1 - unsexy >= 0 {
                last_un[(un + lun - 1 - unsexy) as usize] = i;
            }
            continue;
        }
        if i < lim - 6 && !sv[i as usize] && !sv[(i + 6) as usize] {
            pr += 1;
            // :  ix  last_pr 
            if pr > pairs - lpr {
                let ix = (pr + lpr - 1 - pairs) as usize;
                if ix < last_pr.len() {
                    last_pr[ix][0] = i;
                    last_pr[ix][1] = i + 6;
                }
            }
        } else {
            continue;
        }

        if i < lim - 12 && !sv[(i + 12) as usize] {
            tr += 1;
            if tr > trips - ltr {
                let ix = (tr + ltr - 1 - trips) as usize;
                if ix < last_tr.len() {
                    last_tr[ix][0] = i;
                    last_tr[ix][1] = i + 6;
                    last_tr[ix][2] = i + 12;
                }
            }
        } else {
            continue;
        }

        if i < lim - 18 && !sv[(i + 18) as usize] {
            qd += 1;
            if qd > quads - lqd {
                let ix = (qd + lqd - 1 - quads) as usize;
                if ix < last_qd.len() {
                    last_qd[ix][0] = i;
                    last_qd[ix][1] = i + 6;
                    last_qd[ix][2] = i + 12;
                    last_qd[ix][3] = i + 18;
                }
            }
        } else {
            continue;
        }

        if i < lim - 24 && !sv[(i + 24) as usize] {
            qn += 1;
            if qn > quins - lqn {
                let ix = (qn + lqn - 1 - quins) as usize;
                if ix < last_qn.len() {
                    last_qn[ix][0] = i;
                    last_qn[ix][1] = i + 6;
                    last_qn[ix][2] = i + 12;
                    last_qn[ix][3] = i + 18;
                    last_qn[ix][4] = i + 24;
                }
            }
        }
    }

    print_helper("pairs", pairs, lim, lpr);
    print!("  [");
    for i in 0..lpr {
        print_array(&last_pr[i as usize]);
        // :  \x08  \b
        print!("\x08] ");
    }
    print!("\x08]\n\n");

    print_helper("triplets", trips, lim, ltr);
    print!("  [");
    for i in 0..ltr {
        print_array(&last_tr[i as usize]);
        // :  \x08  \b
        print!("\x08] ");
    }
    print!("\x08]\n\n");

    print_helper("quadruplets", quads, lim, lqd);
    print!("  [");
    for i in 0..lqd {
        print_array(&last_qd[i as usize]);
        // :  \x08  \b
        print!("\x08] ");
    }
    print!("\x08]\n\n");

    print_helper("quintuplets", quins, lim, lqn);
    print!("  [");
    for i in 0..lqn {
        print_array(&last_qn[i as usize]);
        // :  \x08  \b
        print!("\x08] ");
    }
    print!("\x08]\n\n");

    print_helper("unsexy primes", unsexy, lim, lun);
    print!("  [");
    print_array(&last_un);
    // :  \x08  \b
    print!("\x08]\n");
}