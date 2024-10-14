use std::mem;

struct FloatList {
    list: Vec<f32>,
    size: usize,
}

fn floatcmp(a: &f32, b: &f32) -> std::cmp::Ordering {
    if a < b {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Greater
    }
}

fn median(fl: &FloatList) -> f32 {
    let mut list = fl.list.clone();
    list.sort_by(floatcmp);
    let mid = fl.size / 2;
    let mid_minus_one = (fl.size - 1) / 2;
    0.5 * (list[mid] + list[mid_minus_one])
}

fn main() {
    let floats1 = vec![5.1, 2.6, 6.2, 8.8, 4.6, 4.1];
    let flist1 = FloatList {
        list: floats1,
        size: 6,
    };

    let floats2 = vec![5.1, 2.6, 8.8, 4.6, 4.1];
    let flist2 = FloatList {
        list: floats2,
        size: 5,
    };

    println!("flist1 median is {:7.2}", median(&flist1)); // 4.85
    println!("flist2 median is {:7.2}", median(&flist2)); // 4.60
}