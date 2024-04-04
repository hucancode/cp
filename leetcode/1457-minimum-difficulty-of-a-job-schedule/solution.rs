impl Solution {
    pub fn min_difficulty(diff: Vec<i32>, day: i32) -> i32 {
        use std::cmp::{min, max};
        let n = diff.len();
        let mut f = vec![-1;n+1];
        f[0] = 0;
        for _ in 0..day {
            let mut g = vec![-1;n+1];
            for i in (1..=n).rev() {
                if f[i-1] == -1 {
                    continue;
                }
                let mut score = 0;
                for j in i..=n {
                    score = max(score, diff[j-1]);
                    if g[j] == -1 {
                        g[j] = f[i-1] + score;
                    } else {
                        g[j] = min(g[j], f[i-1] + score);
                    }
                }
            }
            f = g;
        }
        f[n]
    }
}
