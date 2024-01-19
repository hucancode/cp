use std::cmp::min;
impl Solution {
    pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
        let mut ret = i32::MAX;
        let n = matrix.len();
        for i in 0..n {
            for j in 0..n {
                if i > 0 {
                    let mut x = matrix[i-1][j];
                    if j > 0 {
                        x = min(x, matrix[i-1][j-1]);
                    }
                    if j < n-1 {
                        x = min(x, matrix[i-1][j+1]);
                    }
                    matrix[i][j] += x;
                }
                if i == n-1 {
                    ret = min(ret, matrix[i][j]);
                }
            }
        }
        ret
    }
}