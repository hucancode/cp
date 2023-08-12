impl Solution {
    pub fn unique_paths_with_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut f = vec![vec![0;m];n];
        for i in 0..n {
            if grid[i][0] == 0 {
                f[i][0] = 1;
            } else {
                break;
            }
        }
        for j in 0..m {
            if grid[0][j] == 0 {
                f[0][j] = 1;
            } else {
                break;
            }
        }
        for i in 1..n {
            for j in 1..m {
                if grid[i][j] == 1 {
                    continue;
                }
                f[i][j] = f[i-1][j] + f[i][j-1];
            }
        }
        return f[n-1][m-1];
    }
}