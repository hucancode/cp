use std::cmp::max;
impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = 1<<n;
        let gcd = |mut x, mut y| {
            while y != 0 {
                let z = x%y;
                x = y;
                y = z;
            }
            x
        };
        let mut g = vec![vec![0;n];n];
        for i in 0..n {
            for j in i+1..n {
                g[i][j] = gcd(nums[i], nums[j]);
            }
        }
        let mut f = vec![0;m];
        for state in (0..m).filter(|x| x.count_ones()%2==0) {
            let multiplier = (state.count_ones()/2 + 1) as i32;
            for i in (0..n).filter(|i| state & (1<<i) == 0) {
                for j in (i+1..n).filter(|j| state & (1<<j) == 0) {
                    let next_state = state | 1<<i | 1<<j;
                    let next_score = multiplier * g[i][j] + f[state];
                    f[next_state] = max(f[next_state], next_score);
                }
            }
        }
        *f.last().unwrap_or(&0)
    }
}