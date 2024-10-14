fn wday(year: i32, month: i32, day: i32) -> i32 {
    let adjustment = (14 - month) / 12;
    let mm = month + 12 * adjustment - 2;
    let yy = year - adjustment;
    (day + (13 * mm - 1) / 5 + yy + yy / 4 - yy / 100 + yy / 400) % 7
}

fn main() {
    for y in 2008..=2121 {
        if wday(y, 12, 25) == 0 {
            println!("{:04}-12-25", y);
        }
    }
}