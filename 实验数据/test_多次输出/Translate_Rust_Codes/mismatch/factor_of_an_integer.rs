use std::mem;

#[derive(Debug)]
struct Factors {
    list: Vec<i32>,
    count: i16,
}

fn xfer_factors(fctrs: &mut Factors, flist: &[i32], flix: usize) {
    let new_size = fctrs.count as usize + flix;
    if new_size > flix {
        fctrs.list.resize(new_size, 0);
    } else {
        fctrs.list = vec![0; new_size];
    }
    for (ij, ix) in (fctrs.count as usize..new_size).enumerate() {
        fctrs.list[ix] = flist[ij];
    }
    fctrs.count = new_size as i16;
}

fn factor(num: i32, fctrs: &mut Factors) {
    let mut flist = [0; 301];
    let mut flix = 0;
    fctrs.count = 0;
    fctrs.list.clear();

    let mut dvsr = 1;
    while dvsr * dvsr < num {
        if num % dvsr == 0 {
            if flix == 300 {
                xfer_factors(fctrs, &flist[..flix], flix);
                flix = 0;
            }
            flist[flix] = dvsr;
            flix += 1;
            flist[flix] = num / dvsr;
            flix += 1;
        }
        dvsr += 1;
    }
    if dvsr * dvsr == num {
        flist[flix] = dvsr;
        flix += 1;
    }
    if flix > 0 {
        xfer_factors(fctrs, &flist[..flix], flix);
    }
}

fn main() {
    let nums_to_factor = [2059, 223092870, 3135, 45];
    let mut ftors = Factors { list: Vec::new(), count: 0 };
    let mut sep;

    for &num in &nums_to_factor {
        factor(num, &mut ftors);
        println!("\nfactors of {} are:", num);
        sep = ' ';
        for &factor in &ftors.list[..ftors.count as usize] {
            print!("{}{}", sep, factor);
            sep = ',';
        }
        println!();
    }
}