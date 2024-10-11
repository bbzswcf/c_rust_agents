fn main() {
    let mut max = 0;
    let mut i = 0;

    loop {
        if i >= 100 {
            break;
        }

        let mut sixes;
        let mut nines;
        let mut twenties;

        for sixes in 0.. {
            if sixes * 6 >= i {
                break;
            }
            if sixes * 6 == i {
                i += 1;
                continue;
            }

            for nines in 0.. {
                if nines * 9 >= i {
                    break;
                }
                if sixes * 6 + nines * 9 == i {
                    i += 1;
                    continue;
                }

                for twenties in 0.. {
                    if twenties * 20 >= i {
                        break;
                    }
                    if sixes * 6 + nines * 9 + twenties * 20 == i {
                        i += 1;
                        continue;
                    }
                }
            }
        }

        max = i;
        i += 1;
    }

    println!("Maximum non-McNuggets number is {}", max);
}