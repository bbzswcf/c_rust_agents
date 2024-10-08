const TRUE: bool = true;
const FALSE: bool = false;

fn bf(lo: i32, hi: i32, unique: bool, show: bool, solutions: &mut i32, a: i32, c: i32, d: i32, e: i32, g: i32) {
    for f in lo..=hi {
        if !unique || (f != a && f != c && f != d && f != g && f != e) {
            let b = e + f - c;
            if b >= lo && b <= hi && (!unique || (b != a && b != c && b != d && b != g && b != e && b != f)) {
                *solutions += 1;
                if show {
                    print!("{} {} {} {} {} {} {}\n", a, b, c, d, e, f, g);
                }
            }
        }
    }
}

fn ge(lo: i32, hi: i32, unique: bool, show: bool, solutions: &mut i32, a: i32, c: i32, d: i32) {
    for e in lo..=hi {
        if !unique || (e != a && e != c && e != d) {
            let g = d + e;
            if g >= lo && g <= hi && (!unique || (g != a && g != c && g != d && g != e)) {
                bf(lo, hi, unique, show, solutions, a, c, d, e, g);
            }
        }
    }
}

fn acd(lo: i32, hi: i32, unique: bool, show: bool, solutions: &mut i32) {
    for c in lo..=hi {
        for d in lo..=hi {
            if !unique || c != d {
                let a = c + d;
                if a >= lo && a <= hi && (!unique || (c != 0 && d != 0)) {
                    ge(lo, hi, unique, show, solutions, a, c, d);
                }
            }
        }
    }
}

fn foursquares(plo: i32, phi: i32, punique: bool, pshow: bool) {
    let mut solutions = 0;
    let lo = plo;
    let hi = phi;
    let unique = punique;
    let show = pshow;

    print!("\n");

    acd(lo, hi, unique, show, &mut solutions);

    if unique {
        print!("\n{} unique solutions in {} to {}\n", solutions, lo, hi);
    } else {
        print!("\n{} non-unique solutions in {} to {}\n", solutions, lo, hi);
    }
}

fn main() {
    foursquares(1, 7, TRUE, TRUE);
    foursquares(3, 9, TRUE, TRUE);
    foursquares(0, 9, FALSE, FALSE);
}