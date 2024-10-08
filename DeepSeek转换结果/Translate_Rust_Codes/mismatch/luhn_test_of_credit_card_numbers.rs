fn luhn(cc: &str) -> bool {
    const M: [u32; 10] = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9]; // mapping for rule 3
    let mut sum = 0;

    // 修改: 确保字符串不为空
    if cc.is_empty() {
        return false;
    }

    // 修改: 使用enumerate()和rev()来遍历字符串
    for (i, ch) in cc.chars().rev().enumerate() {
        if let Some(digit) = ch.to_digit(10) {
            // 修改: 根据Luhn算法规则,对偶数位进行特殊处理
            if i % 2 == 0 {
                sum += digit;
            } else {
                sum += M[((digit * 2) % 10) as usize];
            }
        } else {
            return false; // 修改: 如果字符不是数字,返回false
        }
    }

    sum % 10 == 0
}

fn main() {
    let cc = [
        "49927398716",
        "49927398717",
        "1234567812345678",
        "1234567812345670",
    ];

    for &card in cc.iter() {
        println!("{:16}\t{}", card, if luhn(card) { "ok" } else { "not ok" });
    }
}