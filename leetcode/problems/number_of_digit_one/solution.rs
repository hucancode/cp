impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        //println!("count {n}");
        if n <= 0 {
            return 0;
        }
        let mut base = 1;
        while base*10 <= n {
            base *= 10;
        }
        let m = n/base;
        let remainer = n%base;
        let a = Self::count_digit_one(base-1);
        let b = Self::count_digit_one(remainer);
        return m*a + b + if m == 1 {remainer+1} else {base};
    }
}