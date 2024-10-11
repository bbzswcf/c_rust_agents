fn main() {
    for i in 1..5000 {
        let mut sum = 0;
        let mut number = i;
        
        while number > 0 {
            let digit = number % 10;
            sum += digit.pow(digit as u32);
            number /= 10;
        }
        
        if sum == i {
            println!("{}", i);
        }
    }
}