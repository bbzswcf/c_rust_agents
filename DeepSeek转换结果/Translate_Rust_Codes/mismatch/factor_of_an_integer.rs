struct Factors {
    list: Vec<i32>,
    count: i16,
}

fn xfer_factors(fctrs: &mut Factors, flist: &[i32], flix: i32) {
    let new_size = fctrs.count as usize + flix as usize;
    if new_size > flix as usize {
        fctrs.list.resize(new_size, 0);
    } else {
        fctrs.list = vec![0; new_size];
    }
    for (ij, ix) in (fctrs.count as usize..new_size).enumerate() {
        fctrs.list[ix] = flist[ij];
    }
    fctrs.count = new_size.min(i16::MAX as usize) as i16;
}

fn factor(num: i32, fctrs: &mut Factors) -> &Factors {
    let mut flist = [0; 301];
    let mut flix = 0;
    fctrs.count = 0;
    fctrs.list.clear();
    let mut dvsr = 1;
    while dvsr * dvsr <= num {
        if num % dvsr == 0 {
            if flix == 300 {
                xfer_factors(fctrs, &flist[..flix], flix as i32);
                flix = 0;
            }
            flist[flix] = dvsr;
            flix += 1;
            if dvsr != num / dvsr {
                flist[flix] = num / dvsr;
                flix += 1;
            }
        }
        dvsr += 1;
    }
    if flix > 0 {
        xfer_factors(fctrs, &flist[..flix], flix as i32);
    }
    fctrs.list.sort_unstable();
    fctrs
}

fn main() {
    let nums_to_factor = [2059, 223092870, 3135, 45];
    let mut ftors = Factors { list: Vec::new(), count: 0 };

    for &num in &nums_to_factor {
        factor(num, &mut ftors);
        print!("\nfactors of {} are:\n", num);
        for j in 0..ftors.count as usize {
            if j > 0 {
                print!(", ");
            } else {
                print!("  ");
            }
            print!("{}", ftors.list[j]);
        }
        print!("\n");
    }
}