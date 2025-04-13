impl Solution {
    pub fn count_good_numbers(n: i64) -> i32 {
        let k = 1000_000_007;
        let pow = |mut x, mut n| {
            let mut ret = 1;
            while n > 0 {
                if n%2 == 1 {
                    ret = ret*x%k;
                }
                x = x*x%k;
                n /= 2;
            }
            ret
        };
        (pow(4i64, n/2) * pow(5i64, n-n/2) % k) as i32
    }
}
