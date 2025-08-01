pub fn is_valid(code: &str) -> bool {
    let mut sum = 0;
    let mut count = 0;
    for c in code.chars().rev() {
        if c == ' ' {
            continue;
        }
        if let Some(x) = c.to_digit(10) {
            let mut val = x;
            if count % 2 != 0 {
                val *= 2;
                if val > 9 {
                    val -= 9;
                }
            }
            sum += val;
            count += 1;
        } else {
            return false;
        }
    }
    sum % 10 == 0 && count > 1
}
