fn bf(a: i32, c: i32, d: i32, e: i32, g: i32, lo: i32, hi: i32, unique: bool, show: bool, solutions: &mut i32) {
    for f in lo..=hi {
        if !unique || (f != a && f != c && f != d && f != g && f != e) {
            let b = e + f - c;
            if b >= lo && b <= hi && (!unique || (b != a && b != c && b != d && b != g && b != e && b != f)) {
                *solutions += 1;
                if show {
                    println!("{} {} {} {} {} {} {}", a, b, c, d, e, f, g);
                }
            }
        }
    }
}

fn ge(a: i32, c: i32, d: i32, lo: i32, hi: i32, unique: bool, show: bool, solutions: &mut i32) {
    for e in lo..=hi {
        if !unique || (e != a && e != c && e != d) {
            let g = d + e;
            if g >= lo && g <= hi && (!unique || (g != a && g != c && g != d && g != e)) {
                bf(a, c, d, e, g, lo, hi, unique, show, solutions);
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
                    ge(a, c, d, lo, hi, unique, show, solutions);
                }
            }
        }
    }
}

fn foursquares(lo: i32, hi: i32, unique: bool, show: bool) {
    let mut solutions = 0;

    println!();

    acd(lo, hi, unique, show, &mut solutions);

    if unique {
        println!("\n{} unique solutions in {} to {}", solutions, lo, hi);
    } else {
        println!("\n{} non-unique solutions in {} to {}", solutions, lo, hi);
    }
}

fn main() {
    foursquares(1, 7, true, true);
    foursquares(3, 9, true, true);
    foursquares(0, 9, false, false);
}