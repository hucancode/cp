impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let mut n = matrix.len();
        let mut m = matrix[0].len();
        let mut p = vec![vec![0;m+1];n+1];
        for i in 1..=n {
            for j in 1..=m {
                p[i][j] = p[i-1][j] + p[i][j-1] - p[i-1][j-1] + matrix[i-1][j-1];
            }
        }
        let mut ret = 0;
        for i1 in 1..=n {
            for j1 in 1..=m {
                let a = p[i1][j1];
                for i2 in 0..i1 {
                    for j2 in 0..j1 {
                        let b = p[i2][j2];
                        let c = p[i1][j2];
                        let d = p[i2][j1];
                        if a + b - c - d == target {
                            ret += 1;
                        }
                    }
                }
            }
        }
        return ret;
    }
}