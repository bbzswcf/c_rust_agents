fn reduce(f: fn(i32, i32) -> i32, size: usize, elms: &[i32]) -> i32 {
    // Modified: Changed parameter name from `fn` to `f` to avoid keyword conflict
    let mut val = elms[0];
    for i in 1..size {
        val = f(val, elms[i]); // Modified: Updated expression to use the new parameter name `f`
    }
    val
}

fn add(a: i32, b: i32) -> i32 { a + b }
fn sub(a: i32, b: i32) -> i32 { a - b }
fn mul(a: i32, b: i32) -> i32 { a * b }

fn main() {
    let nums = [1, 2, 3, 4, 5];
    println!("{}", reduce(add, 5, &nums));
    println!("{}", reduce(sub, 5, &nums));
    println!("{}", reduce(mul, 5, &nums));
}