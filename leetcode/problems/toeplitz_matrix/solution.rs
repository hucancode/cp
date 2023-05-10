impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        (1..matrix.len()).all(|i|
            (1..matrix[0].len()).all(|j| 
                matrix[i][j] == matrix[i-1][j-1]
            )
        )
    }
}