impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, mut n: i32) -> i32 {
        let digits: Vec<i32> = digits.into_iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let mut pick_cap_digit = 1;
        let mut pick_all_digit = 1;
        let mut ret = 0;
        while n > 0 {
            let d = n%10;
            n /= 10;
            match digits.binary_search(&d) {
                Ok(j) => pick_cap_digit += j*pick_all_digit,
                Err(j) => pick_cap_digit = j*pick_all_digit,
            }
            pick_all_digit *= digits.len();
            if n == 0 {
                ret += pick_cap_digit;
            } else {
                ret += pick_all_digit;
            }
        }
        ret as i32
    }
}