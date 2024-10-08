struct Range {
    start: i32,
    end: i32,
    sum: i32,
}

fn max_subseq(sequence: &[i32]) -> Range {
    let mut max_sum = 0;
    let mut this_sum = 0;
    let mut i = 0;
    let mut start = 0;
    let mut end = -1;

    for (j, &value) in sequence.iter().enumerate() {
        this_sum += value;
        if this_sum < 0 {
            i = j as i32 + 1;
            this_sum = 0;
        } else if this_sum > max_sum {
            max_sum = this_sum;
            start = i;
            end = j as i32;
        }
    }

    let mut r = Range { start: 0, end: 0, sum: 0 };
    if start <= end && start >= 0 && end >= 0 {
        r.start = start;
        r.end = end + 1;
        r.sum = max_sum;
    }
    r
}

fn main() {
    let a = [-1, -2, 3, 5, 6, -2, -1, 4, -4, 2, -1];
    let alength = a.len();

    let r = max_subseq(&a);
    println!("Max sum = {}", r.sum);
    for i in r.start..r.end {
        print!("{} ", a[i as usize]);
    }
    println!();
}