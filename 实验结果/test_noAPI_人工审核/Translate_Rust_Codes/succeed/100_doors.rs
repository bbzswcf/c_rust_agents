fn main() {
    let mut is_open: [bool; 100] = [false; 100];

    for pass in 0..100 {
        for door in (pass..100).step_by(pass + 1) {
            is_open[door] = !is_open[door];
        }
    }

    for door in 0..100 {
        println!("door #{} is {}.", door + 1, if is_open[door] { "open" } else { "closed" });
    }
}