fn main() {
    let mut max = 0;
    let mut i = 1;

    loop {
        if i >= 100 {
            break;
        }

        let mut found = false;

        'outer: for sixes in 0..=i / 6 {
            for nines in 0..=i / 9 {
                for twenties in 0..=i / 20 {
                    if sixes * 6 + nines * 9 + twenties * 20 == i {
                        found = true;
                        break 'outer;
                    }
                }
            }
        }

        if !found {
            max = i;
        }

        i += 1;
    }

    println!("Maximum non-McNuggets number is {}", max);
}