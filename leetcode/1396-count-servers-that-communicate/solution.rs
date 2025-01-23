impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut online_row = vec![0;n];
        let mut online_col = vec![0;m];
        let mut ret = 0;
        for (i, row) in grid.iter().enumerate() {
            for (j, &x) in row.iter().enumerate() {
                online_row[i] += x;
                online_col[j] += x;
            }
        }
        for (i, row) in grid.iter().enumerate() {
            for (j, &x) in row.iter().enumerate() {
                if online_row[i] > x || online_col[j] > x {
                    ret += x;
                }
            }
        }
        ret
    }
}
