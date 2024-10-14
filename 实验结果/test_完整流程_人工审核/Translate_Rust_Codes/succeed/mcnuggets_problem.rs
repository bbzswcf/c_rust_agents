fn main() {
    let mut max = 0;
    let mut i = 0;
    // Added type annotation to specify the type of the variable `sixes`
    let mut sixes: i32;
    // Added type annotation to specify the type of the variable `nines`
    let mut nines: i32;
    // Added type annotation to specify the type of the variable `twenties`
    let mut twenties: i32;

    'loopstart: while i < 100 {
        for sixes in 0.. {
            if sixes * 6 >= i {
                break;
            }
            if sixes * 6 == i {
                i += 1;
                continue 'loopstart;
            }

            for nines in 0.. {
                if nines * 9 >= i {
                    break;
                }
                if sixes * 6 + nines * 9 == i {
                    i += 1;
                    continue 'loopstart;
                }

                for twenties in 0.. {
                    if twenties * 20 >= i {
                        break;
                    }
                    if sixes * 6 + nines * 9 + twenties * 20 == i {
                        i += 1;
                        continue 'loopstart;
                    }
                }
            }
        }
        max = i;
        i += 1;
    }

    println!("Maximum non-McNuggets number is {}", max);
}