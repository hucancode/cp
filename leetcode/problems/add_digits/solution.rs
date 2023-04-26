impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num < 10 {
            return num;
        }
        let mut ret = 0;
        let mut num = num;
        while num > 0 {
            ret += num%10;
            num /= 10;
        }
        Self::add_digits(ret)
    }
}