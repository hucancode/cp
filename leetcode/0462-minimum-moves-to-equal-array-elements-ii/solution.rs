impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let n = nums.len();
        let mut prefix = vec![0;n+1];
        for i in 1..=n {
            prefix[i] = prefix[i-1]+nums[i-1] as i64;
        }
        let total = prefix[n];
        nums.into_iter()
            .enumerate()
            .zip(prefix.into_iter().skip(1))
            .map(|((i, x), sum)| {
                let cost_l = x as i64 *(i+1) as i64 - sum;
                let cost_r = total - sum - x as i64 * (n-i-1) as i64;
                cost_l + cost_r
            })
            .min()
            .unwrap_or(0) as i32
    }
}
