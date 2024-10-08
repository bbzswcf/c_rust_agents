fn is_leap_year(year: i32) -> bool {
    !(year % 4 != 0 || (year % 100 == 0 && year % 400 != 0))
}

fn main() {
    let test_case: [i32; 5] = [1900, 1994, 1996, 1997, 2000];
    for &year in &test_case {
        println!("{} is {}a leap year.", year, if is_leap_year(year) { "" } else { "not " });
    }
}