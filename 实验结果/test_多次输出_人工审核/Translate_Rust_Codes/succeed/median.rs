fn median(list: &mut [f32]) -> f32 {
    list.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len = list.len();
    let mid = len / 2;
    if len % 2 == 0 {
        (list[mid - 1] + list[mid]) / 2.0
    } else {
        list[mid]
    }
}

fn main() {
    let mut floats1 = vec![5.1, 2.6, 6.2, 8.8, 4.6, 4.1];
    let mut floats2 = vec![5.1, 2.6, 8.8, 4.6, 4.1];

    println!("flist1 median is {:7.2}", median(&mut floats1)); // 4.85
    println!("flist2 median is {:7.2}", median(&mut floats2)); // 4.60
}