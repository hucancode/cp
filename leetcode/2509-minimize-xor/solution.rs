impl Solution {
    pub fn minimize_xor(mut num1: i32, num2: i32) -> i32 {
        let n = num1.count_ones();
        let m = num2.count_ones();
        let mut ret = num1;
        if n >= m {
            let mut take = n - m;
            let mut i = 0;
            while take > 0 {
                if num1 & (1<<i) != 0 {
                    ret ^= 1<<i;
                    take -= 1;
                }
                i += 1;
            }
        } else {
            let mut take = m - n;
            let mut i = 0;
            while take > 0 {
                if num1 & (1<<i) == 0 {
                    ret |= 1<<i;
                    take -= 1;
                }
                i += 1;
            }
        }
        ret
    }
}
