use std::convert::TryInto;

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
    print!(r"\b]"); // Modified: Used raw string literal to avoid unknown character escape
}

fn main() {
    let lim = 1000035;
    let mut pairs = 0;
    let mut trips = 0;
    let mut quads = 0;
    let mut quins = 0;
    let mut unsexy = 2;
    let mut pr: i32 = 0; // Modified: Specified type for `pr`
    let mut tr: i32 = 0; // Modified: Specified type for `tr`
    let mut qd: i32 = 0; // Modified: Specified type for `qd`
    let mut qn: i32 = 0; // Modified: Specified type for `qn`
    let mut un: i32 = 2; // Modified: Specified type for `un`
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
        last_pr = [[0; 2]; 5];
    }
    if trips < ltr {
        last_tr = [[0; 3]; 5];
    }
    if quads < lqd {
        last_qd = [[0; 4]; 5];
    }
    if quins < lqn {
        last_qn = [[0; 5]; 5];
    }
    if unsexy < lun {
        last_un = [0; 10];
    }

    for i in (3..lim).step_by(2) {
        if i > 5 && i < lim - 6 && sv[i] == FALSE && sv[i - 6] == TRUE && sv[i + 6] == TRUE {
            un += 1;
            if un > unsexy - lun {
                if let Some(index) = un.checked_add(lun - 1).and_then(|sum| sum.checked_sub(unsexy)) {
                    if index < last_un.len().try_into().unwrap() { // Modified: Convert `last_un.len()` to `i32`
                        last_un[index.try_into().unwrap()] = i as i32; // Modified: Convert `index` to `usize`
                    }
                }
            }
            continue;
        }
        if i < lim - 6 && sv[i] == FALSE && sv[i + 6] == FALSE {
            pr += 1;
            if pr > pairs - lpr {
                if let Some(ix) = pr.checked_add(lpr - 1).and_then(|sum| sum.checked_sub(pairs)) {
                    if ix < last_pr.len().try_into().unwrap() { // Modified: Convert `last_pr.len()` to `i32`
                        last_pr[ix.try_into().unwrap()][0] = i as i32; // Modified: Convert `ix` to `usize`
                        last_pr[ix.try_into().unwrap()][1] = (i + 6) as i32; // Modified: Convert `ix` to `usize`
                    }
                }
            }
        } else {
            continue;
        }

        if i < lim - 12 && sv[i + 12] == FALSE {
            tr += 1;
            if tr > trips - ltr {
                if let Some(ix) = tr.checked_add(ltr - 1).and_then(|sum| sum.checked_sub(trips)) {
                    if ix < last_tr.len().try_into().unwrap() { // Modified: Convert `last_tr.len()` to `i32`
                        last_tr[ix.try_into().unwrap()][0] = i as i32; // Modified: Convert `ix` to `usize`
                        last_tr[ix.try_into().unwrap()][1] = (i + 6) as i32; // Modified: Convert `ix` to `usize`
                        last_tr[ix.try_into().unwrap()][2] = (i + 12) as i32; // Modified: Convert `ix` to `usize`
                    }
                }
            }
        } else {
            continue;
        }

        if i < lim - 18 && sv[i + 18] == FALSE {
            qd += 1;
            if qd > quads - lqd {
                if let Some(ix) = qd.checked_add(lqd - 1).and_then(|sum| sum.checked_sub(quads)) {
                    if ix < last_qd.len().try_into().unwrap() { // Modified: Convert `last_qd.len()` to `i32`
                        last_qd[ix.try_into().unwrap()][0] = i as i32; // Modified: Convert `ix` to `usize`
                        last_qd[ix.try_into().unwrap()][1] = (i + 6) as i32; // Modified: Convert `ix` to `usize`
                        last_qd[ix.try_into().unwrap()][2] = (i + 12) as i32; // Modified: Convert `ix` to `usize`
                        last_qd[ix.try_into().unwrap()][3] = (i + 18) as i32; // Modified: Convert `ix` to `usize`
                    }
                }
            }
        } else {
            continue;
        }

        if i < lim - 24 && sv[i + 24] == FALSE {
            qn += 1;
            if qn > quins - lqn {
                if let Some(ix) = qn.checked_add(lqn - 1).and_then(|sum| sum.checked_sub(quins)) {
                    if ix < last_qn.len().try_into().unwrap() { // Modified: Convert `last_qn.len()` to `i32`
                        last_qn[ix.try_into().unwrap()][0] = i as i32; // Modified: Convert `ix` to `usize`
                        last_qn[ix.try_into().unwrap()][1] = (i + 6) as i32; // Modified: Convert `ix` to `usize`
                        last_qn[ix.try_into().unwrap()][2] = (i + 12) as i32; // Modified: Convert `ix` to `usize`
                        last_qn[ix.try_into().unwrap()][3] = (i + 18) as i32; // Modified: Convert `ix` to `usize`
                        last_qn[ix.try_into().unwrap()][4] = (i + 24) as i32; // Modified: Convert `ix` to `usize`
                    }
                }
            }
        }
    }

    print_helper("pairs", pairs.try_into().unwrap(), lim, lpr.try_into().unwrap()); // Modified: Convert `pairs` and `lpr` to `usize`
    print!("  [");
    for i in 0..lpr {
        print_array(&last_pr[i.try_into().unwrap()]); // Modified: Convert `i` to `usize`
        print!(r"\b] "); // Modified: Used raw string literal to avoid unknown character escape
    }
    print!(r"\b]\n\n"); // Modified: Used raw string literal to avoid unknown character escape

    print_helper("triplets", trips.try_into().unwrap(), lim, ltr.try_into().unwrap()); // Modified: Convert `trips` and `ltr` to `usize`
    print!("  [");
    for i in 0..ltr {
        print_array(&last_tr[i.try_into().unwrap()]); // Modified: Convert `i` to `usize`
        print!(r"\b] "); // Modified: Used raw string literal to avoid unknown character escape
    }
    print!(r"\b]\n\n"); // Modified: Used raw string literal to avoid unknown character escape

    print_helper("quadruplets", quads.try_into().unwrap(), lim, lqd.try_into().unwrap()); // Modified: Convert `quads` and `lqd` to `usize`
    print!("  [");
    for i in 0..lqd {
        print_array(&last_qd[i.try_into().unwrap()]); // Modified: Convert `i` to `usize`
        print!(r"\b] "); // Modified: Used raw string literal to avoid unknown character escape
    }
    print!(r"\b]\n\n"); // Modified: Used raw string literal to avoid unknown character escape

    print_helper("quintuplets", quins.try_into().unwrap(), lim, lqn.try_into().unwrap()); // Modified: Convert `quins` and `lqn` to `usize`
    print!("  [");
    for i in 0..lqn {
        print_array(&last_qn[i.try_into().unwrap()]); // Modified: Convert `i` to `usize`
        print!(r"\b] "); // Modified: Used raw string literal to avoid unknown character escape
    }
    print!(r"\b]\n\n"); // Modified: Used raw string literal to avoid unknown character escape

    print_helper("unsexy primes", unsexy.try_into().unwrap(), lim, lun.try_into().unwrap()); // Modified: Convert `unsexy` and `lun` to `usize`
    print!("  [");
    print_array(&last_un);
    print!(r"\b]\n"); // Modified: Used raw string literal to avoid unknown character escape
}