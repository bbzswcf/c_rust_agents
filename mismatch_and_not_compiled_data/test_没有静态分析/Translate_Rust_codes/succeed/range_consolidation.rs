use std::mem;

#[derive(Debug)]
struct Range {
    low: f64,
    high: f64,
}

fn normalize_range(range: &mut Range) {
    if range.high < range.low {
        std::mem::swap(&mut range.low, &mut range.high);
    }
}

fn range_compare(r1: &Range, r2: &Range) -> std::cmp::Ordering {
    if r1.low < r2.low {
        std::cmp::Ordering::Less
    } else if r1.low > r2.low {
        std::cmp::Ordering::Greater
    } else if r1.high < r2.high {
        std::cmp::Ordering::Less
    } else if r1.high > r2.high {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Equal
    }
}

fn normalize_ranges(ranges: &mut [Range]) {
    for range in ranges.iter_mut() {
        normalize_range(range);
    }
    ranges.sort_by(range_compare);
}

fn consolidate_ranges(ranges: &mut [Range]) -> usize {
    normalize_ranges(ranges);
    let mut out_index = 0;
    let mut i = 0;
    while i < ranges.len() {
        let mut j = i;
        while j + 1 < ranges.len() && ranges[j + 1].low <= ranges[i].high {
            j += 1;
            if ranges[i].high < ranges[j].high {
                ranges[i].high = ranges[j].high;
            }
        }
        ranges.swap(out_index, i);
        out_index += 1;
        i = j + 1;
    }
    out_index
}

fn print_range(range: &Range) {
    print!("[{}, {}]", range.low, range.high);
}

fn print_ranges(ranges: &[Range]) {
    if ranges.is_empty() {
        return;
    }
    print_range(&ranges[0]);
    for range in &ranges[1..] {
        print!(", ");
        print_range(range);
    }
}

fn test_consolidate_ranges(ranges: &mut [Range]) {
    print_ranges(ranges);
    print!(" -> ");
    let count = consolidate_ranges(ranges);
    print_ranges(&ranges[..count]);
    println!();
}

fn main() {
    let mut test1 = vec![Range { low: 1.1, high: 2.2 }];
    let mut test2 = vec![Range { low: 6.1, high: 7.2 }, Range { low: 7.2, high: 8.3 }];
    let mut test3 = vec![Range { low: 4.0, high: 3.0 }, Range { low: 2.0, high: 1.0 }];
    let mut test4 = vec![Range { low: 4.0, high: 3.0 }, Range { low: 2.0, high: 1.0 }, Range { low: -1.0, high: -2.0 }, Range { low: 3.9, high: 10.0 }];
    let mut test5 = vec![Range { low: 1.0, high: 3.0 }, Range { low: -6.0, high: -1.0 }, Range { low: -4.0, high: -5.0 }, Range { low: 8.0, high: 2.0 }, Range { low: -6.0, high: -6.0 }];

    test_consolidate_ranges(&mut test1);
    test_consolidate_ranges(&mut test2);
    test_consolidate_ranges(&mut test3);
    test_consolidate_ranges(&mut test4);
    test_consolidate_ranges(&mut test5);
}