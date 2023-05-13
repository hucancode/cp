impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let low = low as usize;
        let high = high as usize;
        let zero = zero as usize;
        let one = one as usize;
        const INF: i32 = 1000_000_007;
        let mut f = vec![0; high+1];
        f[0] = 1;
        for i in 0..=high {
            let j = i + zero;
            if j <= high {
                f[j] += f[i];
                f[j] %= INF;
            }
            let j = i + one;
            if j <= high {
                f[j] += f[i];
                f[j] %= INF;
            }
        }
        f.iter()
            .skip(low)
            .fold(0, |acc, x| (acc + x)%INF)
    }
}