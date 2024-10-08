struct Factors {
    list: Vec<i32>,
    count: usize,
}

fn xfer_factors(fctrs: &mut Factors, flist: &[i32], flix: usize) {
    let new_size = fctrs.count + flix;
    if new_size > fctrs.list.len() {
        fctrs.list.resize(new_size, 0); // Resize the vector once before starting the factorization process
    }
    for (ij, ix) in (fctrs.count..new_size).enumerate() {
        fctrs.list[ix] = flist[ij];
    }
    fctrs.count = new_size;
}

fn factor(num: i32, fctrs: &mut Factors) {
    let mut flist = Vec::new();
    fctrs.count = 0;

    let mut dvsr = 1;
    while dvsr * dvsr <= num {
        if num % dvsr == 0 {
            flist.push(dvsr);
            if dvsr != num / dvsr {
                flist.push(num / dvsr);
            }
        }
        dvsr += 1;
    }
    xfer_factors(fctrs, &flist, flist.len());
}

fn main() {
    let nums_to_factor = [2059, 223092870, 3135, 45];
    let mut ftors = Factors { list: Vec::new(), count: 0 };

    for &num in &nums_to_factor {
        factor(num, &mut ftors);
        print!("\nfactors of {} are:\n  ", num);
        for j in 0..ftors.count / 2 {
            if j != 0 {
                print!(", ");
            }
            print!("{} {}", ftors.list[j], ftors.list[ftors.count - 1 - j]); // Correctly pair factors from the other end of the list
        }
        if ftors.count % 2 != 0 {
            print!(" {}", ftors.list[ftors.count / 2]); // Print the last factor correctly if the number of factors is odd
        }
        println!();
    }
}