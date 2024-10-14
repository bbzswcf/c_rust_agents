use std::mem;

#[derive(Debug)]
struct Factors {
    list: Vec<i32>,
    count: i16,
}

impl Factors {
    fn xfer_factors(&mut self, flist: &[i32], flix: usize) {
        let new_size = self.count as usize + flix;
        if new_size > self.list.len() {
            self.list.resize(new_size, 0);
        }
        for (ij, ix) in (self.count as usize..new_size).enumerate() {
            self.list[ix] = flist[ij];
        }
        self.count = new_size as i16;
    }

    fn factor(&mut self, num: i32) {
        let mut flist = [0; 301];
        let mut flix = 0;
        self.count = 0;
        self.list.clear();

        let mut dvsr = 1;
        while dvsr * dvsr < num {
            if num % dvsr == 0 {
                if flix == 300 {
                    self.xfer_factors(&flist[..flix], flix);
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
            self.xfer_factors(&flist[..flix], flix);
        }
    }
}

fn main() {
    let nums_to_factor = [2059, 223092870, 3135, 45];
    let mut ftors = Factors { list: Vec::new(), count: 0 };
    let mut sep;

    for &num in &nums_to_factor {
        ftors.factor(num);
        println!("\nfactors of {} are:", num);
        sep = ' ';
        for &factor in &ftors.list[..ftors.count as usize] {
            print!("{}{}", sep, factor);
            sep = ',';
        }
        println!();
    }
}