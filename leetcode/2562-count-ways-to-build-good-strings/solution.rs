impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let INF = 1000_000_007;
        let low = low as usize;
        let high = high as usize;
        let zero = zero as usize;
        let one = one as usize;
        let mut f = vec![0;high + 1];
        f[0] = 1;
        for i in 1..=high {
            if i >= zero {
                f[i] += f[i-zero];
            }
            if i >= one {
                f[i] += f[i-one];
            }
            f[i] %= INF;
        }
        f[low..].iter()
            .fold(0, |acc, x| (acc + x)%INF) as i32
    }
}
