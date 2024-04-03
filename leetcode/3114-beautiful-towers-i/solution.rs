impl Solution {
    pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
        use std::cmp::{max, min};
        let n = max_heights.len();
        let mut f = vec![0;n];
        let mut g = vec![0;n];
        for i in 0..n {
            let mut x = max_heights[i];
            f[i] += x as i64;
            for j in (0..i).rev() {
                if max_heights[j] <= x {
                    f[i] += f[j];
                    break;
                }
                x = min(x, max_heights[j]);
                f[i] += x as i64;
            }
        }
        for i in (0..n).rev() {
            let mut x = max_heights[i];
            g[i] += x as i64;
            for j in i+1..n {
                if max_heights[j] <= x {
                    g[i] += g[j];
                    break;
                }
                x = min(x, max_heights[j]);
                g[i] += x as i64;
            }
        }
        (0..n).map(|i| g[i] + f[i] - max_heights[i] as i64)
            .max()
            .unwrap_or(0)
    }
}
