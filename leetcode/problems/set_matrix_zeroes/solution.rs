impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let m = matrix[0].len();
        let should_fill_row0 = matrix[0]
            .iter()
            .any(|&x| x == 0);
        let should_fill_col0 = matrix
            .iter()
            .map(|row| row[0])
            .any(|x| x == 0);
        for i in 1..n {
            let has_0 = matrix[i]
                .iter()
                .any(|&x| x == 0);
            if has_0 {
                matrix[i][0] = 0;
            }
        }
        for j in 1..m {
            let has_0 = matrix
                .iter()
                .map(|row| row[j])
                .any(|x| x == 0);
            if has_0 {
                matrix[0][j] = 0;
            }
        }
        for i in 1..n {
            for j in 1..m {
                if matrix[0][j] == 0 || matrix[i][0] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }
        if should_fill_row0 {
            for j in 0..m {
                matrix[0][j] = 0;
            }
        }
        if should_fill_col0 {
            for i in 0..n {
                matrix[i][0] = 0;
            }
        }
    }
}