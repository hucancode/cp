impl Solution {
    pub fn grid_game(mut grid: Vec<Vec<i32>>) -> i64 {
        let n = grid[0].len();
        let mut suffix = vec![0;n+1];
        let mut prefix = vec![0;n+1];
        for i in (0..n).rev() {
            suffix[i] = suffix[i+1] + grid[0][i] as i64;
        }
        for i in 0..n {
            prefix[i+1] = prefix[i] + grid[1][i] as i64;
        }
        (0..n).map(|i| prefix[i].max(suffix[i+1]))
            .min()
            .unwrap_or(0)
    }
}
