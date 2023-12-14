impl Solution {
    pub fn ones_minus_zeros(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let m = grid[0].len();
        let nm = n+m;
        let mut cols = vec![0;n];
        let mut rows = vec![0;m];
        for i in 0..n {
            for j in 0..m {
                cols[i] += grid[i][j];
                rows[j] += grid[i][j];
            }
        }
        let mut ret = 0;
        for i in 0..n {
            for j in 0..m {
                grid[i][j] = 2*(cols[i]+rows[j]) - nm as i32;
            }
        }
        return grid;
    }
}