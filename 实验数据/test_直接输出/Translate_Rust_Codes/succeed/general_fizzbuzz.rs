use std::slice;

struct ReplaceInfo {
    n: i32,
    text: String,
}

fn compare(a: &ReplaceInfo, b: &ReplaceInfo) -> std::cmp::Ordering {
    a.n.cmp(&b.n)
}

fn generic_fizz_buzz(max: i32, info: &[ReplaceInfo]) {
    for i in 1..max {
        let mut found_word = false;

        for item in info {
            if i % item.n == 0 {
                print!("{}", item.text);
                found_word = true;
            }
        }

        if !found_word {
            print!("{}", i);
        }

        println!();
    }
}

fn main() {
    let mut info = vec![
        ReplaceInfo { n: 5, text: "Buzz".to_string() },
        ReplaceInfo { n: 7, text: "Baxx".to_string() },
        ReplaceInfo { n: 3, text: "Fizz".to_string() },
    ];

    info.sort_by(compare);

    generic_fizz_buzz(20, &info);
}