impl Solution {
    pub fn matrix_score(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut flip_row = vec![0;n];
        let mut flip_column = vec![0;m];
        for i in 0..n {
            if grid[i][0] == 0 {
                flip_row[i] = 1;
            }
        }
        for j in 0..m {
            let one_count: i32 = (0..n).map(|i| (grid[i][j] + flip_row[i])%2).sum();
            if one_count*2 < (n as i32) {
                flip_column[j] =  1;
            }
        }
        let mut ret = 0;
        for i in 0..n {
            let mut r = 0;
            for j in 0..m {
                let mut x = grid[i][j] + flip_row[i] + flip_column[j];
                r = (r<<1) + (x & 1);
            }
            ret += r;
        }
        ret
    }
}
