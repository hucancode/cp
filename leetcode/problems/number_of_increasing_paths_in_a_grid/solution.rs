use std::cmp::Reverse;
use std::collections::BinaryHeap;
const INF: i32 = 1000_000_007;
impl Solution {
    pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut f = vec![vec![1;m];n];
        let mut q = BinaryHeap::new();
        for i in 0..n {
            for j in 0..m {
                q.push((Reverse(grid[i][j]), i, j));
            }
        }
        while let Some((Reverse(v), i, j)) = q.pop() {
            if i > 0 && grid[i-1][j] < grid[i][j] {
                f[i][j] += f[i-1][j];
                f[i][j] %= INF;
            }
            if i < n - 1 && grid[i+1][j] < grid[i][j] {
                f[i][j] += f[i+1][j];
                f[i][j] %= INF;
            }
            if j > 0 && grid[i][j-1] < grid[i][j] {
                f[i][j] += f[i][j-1];
                f[i][j] %= INF;
            }
            if j < m - 1 && grid[i][j+1] < grid[i][j] {
                f[i][j] += f[i][j+1];
                f[i][j] %= INF;
            }
        }
        let mut ret = 0;
        for i in 0..n {
            for j in 0..m {
                ret += f[i][j];
                ret %= INF;
            }
        }
        ret
    }
}