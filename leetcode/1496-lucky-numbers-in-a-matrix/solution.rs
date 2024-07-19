impl Solution {
    pub fn lucky_numbers (matrix: Vec<Vec<i32>>) -> Vec<i32> {
        use std::cmp::{min, max};
        let n = matrix.len();
        let m = matrix[0].len();
        let mut r = vec![i32::MAX;n];
        let mut c = vec![i32::MIN;m];
        for i in 0..n {
            for j in 0..m {
                c[j] = max(matrix[i][j], c[j]);
                r[i] = min(matrix[i][j], r[i]);
            }
        }
        let mut ret = Vec::new();
        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == c[j] && matrix[i][j] == r[i] {
                    ret.push(matrix[i][j]);
                }
            }
        }
        ret
    }
}
