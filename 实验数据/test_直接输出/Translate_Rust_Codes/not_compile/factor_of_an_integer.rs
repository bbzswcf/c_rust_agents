use std::mem;

struct Factors {
    list: Vec<i32>,
    count: i16,
}

fn xfer_factors(fctrs: &mut Factors, flist: &[i32], flix: i16) {
    let new_size = fctrs.count + flix;
    if new_size > flix {
        fctrs.list.resize(new_size as usize, 0);
    } else {
        fctrs.list = Vec::with_capacity(new_size as usize);
    }
    for (ij, ix) in (fctrs.count..new_size).enumerate() {
        fctrs.list[ix as usize] = flist[ij as usize];
    }
    fctrs.count = new_size;
}

fn factor(num: i32, fctrs: &mut Factors) -> &mut Factors {
    let mut flist = [0; 301];
    let mut flix = 0;
    fctrs.count = 0;
    fctrs.list.clear();

    let mut dvsr = 1;
    while dvsr * dvsr < num {
        if num % dvsr == 0 {
            if flix == 300 {
                xfer_factors(fctrs, &flist[..flix as usize], flix);
                flix = 0;
            }
            flist[flix as usize] = dvsr;
            flix += 1;
            flist[flix as usize] = num / dvsr;
            flix += 1;
        }
        dvsr += 1;
    }
    if dvsr * dvsr == num {
        flist[flix as usize] = dvsr;
        flix += 1;
    }
    if flix > 0 {
        xfer_factors(fctrs, &flist[..flix as usize], flix);
    }

    fctrs
}

fn main() {
    let nums_to_factor = [2059, 223092870, 3135, 45];
    let mut ftors = Factors { list: Vec::new(), count: 0 };
    let mut sep;

    for &num in &nums_to_factor {
        factor(num, &mut ftors);
        println!("\nfactors of {} are:\n  ", num);
        sep = ' ';
        for j in 0..ftors.count {
            print!("{}{}", sep, ftors.list[j as usize]);
            sep = ',';
        }
        println!();
    }
}