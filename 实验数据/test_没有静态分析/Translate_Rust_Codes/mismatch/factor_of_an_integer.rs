struct Factors {
    list: Vec<i32>,
    count: i16,
}

fn xfer_factors(fctrs: &mut Factors, flist: &[i32], flix: i16) {
    // Ensure the vector is resized correctly to avoid out-of-bounds access
    let new_size = fctrs.count.checked_add(flix).expect("Overflow in new_size calculation");
    fctrs.list.resize(new_size as usize, 0);
    for (ij, &factor) in flist.iter().enumerate().take(flix as usize) {
        // Convert fctrs.count to usize before performing the addition
        fctrs.list[(fctrs.count as usize + ij)] = factor;
    }
    fctrs.count = new_size;
}

fn factor(num: i32, fctrs: &mut Factors) -> &Factors {
    let mut flist = [0; 301];
    let mut flix = 0;
    fctrs.count = 0;
    fctrs.list.clear();

    // Reserve a reasonable capacity based on the expected number of factors
    fctrs.list.reserve(30);

    // Modified: Specify the type of `dvsr` to resolve the ambiguity
    let mut dvsr: i32 = 1;
    // Use checked division to avoid potential overflow
    while dvsr <= num.checked_div(dvsr).expect("Division by zero or overflow") {
        if num % dvsr == 0 {
            // Ensure the factor list is not transferred prematurely
            if flix >= 300 {
                xfer_factors(fctrs, &flist[..flix as usize], flix);
                flix = 0;
            }
            flist[flix as usize] = dvsr;
            flix += 1;
            if dvsr != num / dvsr {
                flist[flix as usize] = num / dvsr;
                flix += 1;
            }
        }
        dvsr += 1;
    }
    // Correctly handle perfect squares by adding the square root only once
    if dvsr.checked_mul(dvsr) == Some(num) {
        flist[flix as usize] = dvsr;
        flix += 1;
    }
    if flix > 0 {
        xfer_factors(fctrs, &flist[..flix as usize], flix);
        fctrs.list.sort();
    }

    fctrs
}

fn main() {
    let nums_to_factor = [2059, 223092870, 3135, 45];
    let mut ftors = Factors { list: Vec::new(), count: 0 };

    for &num in &nums_to_factor {
        factor(num, &mut ftors);
        println!("\nfactors of {} are:\n  ", num);
        // Ensure the factors are printed correctly with commas and spaces
        for (j, &factor) in ftors.list.iter().enumerate().take(ftors.count as usize) {
            if j > 0 {
                print!(", ");
            }
            print!("{}", factor);
        }
        println!();
    }
}