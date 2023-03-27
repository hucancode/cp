use std::cmp::min;
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut f = vec![vec![0;m];n];
        for i in 0..n {
            for j in 0..m {
                f[i][j] = grid[i][j];
                if i > 0 && j > 0 {
                    f[i][j] += min(f[i-1][j], f[i][j-1]);
                    continue;
                }
                if i > 0 {
                    f[i][j] += f[i-1][j];
                    continue;
                }
                if j > 0 {
                    f[i][j] += f[i][j-1];
                    continue;
                }
            }
        }
        return f[n-1][m-1];
    }
}