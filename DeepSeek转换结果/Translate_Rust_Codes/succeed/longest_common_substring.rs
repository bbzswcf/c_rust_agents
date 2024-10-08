fn lcs(sa: &str, sb: &str, beg: &mut String, end: &mut String) {
    let sa_bytes = sa.as_bytes();
    let sb_bytes = sb.as_bytes();
    let mut max_len = 0;
    let mut max_end = 0;

    // 初始化 beg 和 end
    *beg = String::new();
    *end = String::new();

    // 创建动态规划表
    let mut dp = vec![vec![0; sb.len() + 1]; sa.len() + 1];

    for (i, &a_char) in sa_bytes.iter().enumerate() {
        for (j, &b_char) in sb_bytes.iter().enumerate() {
            if a_char == b_char {
                dp[i + 1][j + 1] = dp[i][j] + 1;
                if dp[i + 1][j + 1] > max_len {
                    max_len = dp[i + 1][j + 1];
                    max_end = i + 1;
                }
            }
        }
    }

    // 根据 max_len 和 max_end 更新 end
    if max_len > 0 {
        *end = sa[max_end - max_len..max_end].to_string();
    }
}

fn main() {
    let s1 = "thisisatest";
    let s2 = "testing123testing";
    let mut beg = String::new();
    let mut end = String::new();

    lcs(s1, s2, &mut beg, &mut end);

    // 输出 end
    if !end.is_empty() {
        println!("{}", end);
    }
}