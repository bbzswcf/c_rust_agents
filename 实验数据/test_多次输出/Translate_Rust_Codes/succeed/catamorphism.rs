fn reduce<F>(func: F, size: usize, elms: &[i32]) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    let mut val = elms[0];
    for i in 1..size {
        val = func(val, elms[i]);
    }
    val
}

fn add(a: i32, b: i32) -> i32 { a + b }
fn sub(a: i32, b: i32) -> i32 { a - b }
fn mul(a: i32, b: i32) -> i32 { a * b }

fn main() {
    let nums = [1, 2, 3, 4, 5];
    println!("{}", reduce(add, nums.len(), &nums));
    println!("{}", reduce(sub, nums.len(), &nums));
    println!("{}", reduce(mul, nums.len(), &nums));
}