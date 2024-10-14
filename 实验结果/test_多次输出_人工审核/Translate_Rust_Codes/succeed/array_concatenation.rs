fn array_concat<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Clone,
{
    let mut result = Vec::with_capacity(a.len() + b.len());
    result.extend_from_slice(a);
    result.extend_from_slice(b);
    result
}

fn main() {
    let a = [1, 2, 3, 4, 5];
    let b = [6, 7, 8, 9, 0];

    let c = array_concat(&a, &b);

    for &item in &c {
        println!("{}", item);
    }
}