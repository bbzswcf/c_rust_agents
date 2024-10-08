fn levenshtein(s: &str, ls: usize, t: &str, lt: usize) -> i32 {
    // 修改: 确保 ls 和 lt 不超过字符串的长度
    if ls > s.chars().count() || lt > t.chars().count() {
        return (ls.max(lt)) as i32;
    }

    if ls == 0 {
        return lt as i32;
    }
    if lt == 0 {
        return ls as i32;
    }

    // 修改: 使用 chars().count() 而不是 len() 来获取字符数
    if ls > 0 && lt > 0 {
        if let (Some(sc), Some(tc)) = (s.chars().nth(ls - 1), t.chars().nth(lt - 1)) {
            if sc == tc {
                return levenshtein(s, ls - 1, t, lt - 1);
            }
        }
    }

    // 修改: 使用 chars().count() 而不是 len() 来获取字符数
    if let (Some(sc), Some(tc)) = (s.chars().nth(ls - 1), t.chars().nth(lt - 1)) {
        if sc == tc {
            return levenshtein(s, ls - 1, t, lt - 1);
        }
    }

    let mut a = levenshtein(s, ls - 1, t, lt - 1);
    let b = levenshtein(s, ls, t, lt - 1);
    let c = levenshtein(s, ls - 1, t, lt);

    if a > b {
        a = b;
    }
    if a > c {
        a = c;
    }

    a + 1
}

fn main() {
    let s1 = "rosettacode";
    let s2 = "raisethysword";
    // 修改: 使用 chars().count() 而不是 len() 来获取字符数
    println!("distance between `{}' and `{}': {}", s1, s2, levenshtein(s1, s1.chars().count(), s2, s2.chars().count()));
}