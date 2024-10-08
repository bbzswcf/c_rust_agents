#[derive(Debug, Clone, Copy)]
struct Range {
    low: f64,
    high: f64,
}

fn normalize_range(range: &mut Range) {
    // Modified: Use EPSILON for comparison to handle floating-point precision issues
    if range.high - range.low < -EPSILON {
        std::mem::swap(&mut range.low, &mut range.high); // Ensure low is always less than or equal to high
    }
}

const EPSILON: f64 = 1e-9;

fn range_compare(r1: &Range, r2: &Range) -> std::cmp::Ordering {
    if (r1.low - r2.low).abs() < EPSILON {
        if (r1.high - r2.high).abs() < EPSILON {
            std::cmp::Ordering::Equal
        } else if r1.high < r2.high {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    } else if r1.low < r2.low {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Greater
    }
}

fn normalize_ranges(ranges: &mut [Range]) {
    for range in ranges.iter_mut() {
        normalize_range(range);
    }
    ranges.sort_by(range_compare); // Ensure ranges are sorted primarily by low and secondarily by high
}

fn consolidate_ranges(ranges: &mut [Range]) -> usize {
    normalize_ranges(ranges);
    let mut out_index = 0;
    let mut i = 0;
    while i < ranges.len() {
        let mut j = i;
        // Modified: Corrected the condition to check for overlapping ranges using EPSILON
        while j + 1 < ranges.len() && ranges[j + 1].low - ranges[j].high < EPSILON {
            j += 1;
            // Ensure the high value is updated correctly for all overlapping scenarios
            ranges[i].high = ranges[j].high.max(ranges[i].high);
        }
        ranges[out_index] = ranges[i];
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
    print!("[");
    print_range(&ranges[0]);
    for range in &ranges[1..] {
        print!(", "); // Added comma and space between ranges
        print_range(range);
    }
    print!("]");
}

fn test_consolidate_ranges(ranges: &mut [Range]) {
    print_ranges(ranges);
    print!(" -> ");
    let count = consolidate_ranges(ranges);
    print_ranges(&ranges[..count]);
    println!();
}

fn main() {
    let mut test1 = [Range { low: 1.1, high: 2.2 }];
    let mut test2 = [Range { low: 6.1, high: 7.2 }, Range { low: 7.2, high: 8.3 }];
    let mut test3 = [Range { low: 4.0, high: 3.0 }, Range { low: 2.0, high: 1.0 }]; // Corrected low-high values
    let mut test4 = [
        Range { low: 4.0, high: 3.0 },
        Range { low: 2.0, high: 1.0 },
        Range { low: -1.0, high: -2.0 },
        Range { low: 3.9, high: 10.0 },
    ]; // Corrected low-high values
    let mut test5 = [
        Range { low: 1.0, high: 3.0 },
        Range { low: -6.0, high: -1.0 },
        Range { low: -4.0, high: -5.0 },
        Range { low: 8.0, high: 2.0 },
        Range { low: -6.0, high: -6.0 },
    ]; // Corrected low-high values

    test_consolidate_ranges(&mut test1);
    test_consolidate_ranges(&mut test2);
    test_consolidate_ranges(&mut test3);
    test_consolidate_ranges(&mut test4);
    test_consolidate_ranges(&mut test5);
}