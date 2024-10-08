fn array_concat<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Copy,
{
    let an = a.len();
    let bn = b.len();
    let mut result = Vec::with_capacity(an + bn);
    unsafe {
        std::ptr::copy_nonoverlapping(a.as_ptr(), result.as_mut_ptr(), an);
        std::ptr::copy_nonoverlapping(b.as_ptr(), result.as_mut_ptr().add(an), bn);
        result.set_len(an + bn);
    }
    result
}

fn main() {
    let a = [1, 2, 3, 4, 5];
    let b = [6, 7, 8, 9, 0];

    let c = array_concat(&a, &b);

    for i in 0..10 {
        println!("{}", c[i]);
    }
}