#[derive(Debug, Clone)]
struct ReplaceInfo {
    n: i32,
    text: String,
}

impl PartialEq for ReplaceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n
    }
}

impl Eq for ReplaceInfo {}

impl PartialOrd for ReplaceInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.n.partial_cmp(&other.n)
    }
}

impl Ord for ReplaceInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.n.cmp(&other.n)
    }
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

    info.sort();

    generic_fizz_buzz(20, &info);
}