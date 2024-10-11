struct Range {
    start: usize,
    end: usize,
    sum: i32,
}

fn max_subseq(sequence: &[i32]) -> Range {
    let mut max_sum = 0;
    let mut this_sum = 0;
    let mut start = 0;
    let mut end = 0;
    let mut i = 0;

    for (j, &value) in sequence.iter().enumerate() {
        this_sum += value;
        if this_sum < 0 {
            i = j + 1;
            this_sum = 0;
        } else if this_sum > max_sum {
            max_sum = this_sum;
            start = i;
            end = j;
        }
    }

    // Modified: Removed redundant comparisons `start >= 0` and `end >= 0`
    if start <= end {
        Range {
            start,
            end: end + 1,
            sum: max_sum,
        }
    } else {
        Range {
            start: 0,
            end: 0,
            sum: 0,
        }
    }
}

fn main() {
    let a = [-1, -2, 3, 5, 6, -2, -1, 4, -4, 2, -1];
    let alength = a.len();

    let r = max_subseq(&a);
    println!("Max sum = {}", r.sum);

    // Modified: Used an iterator instead of a range loop
    for &value in a.iter().skip(r.start).take(r.end - r.start) {
        print!("{} ", value);
    }
    println!();
}