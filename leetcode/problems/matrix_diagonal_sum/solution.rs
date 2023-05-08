impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        (0..n)
            .map(|i|
                mat[i][i] + if n-1-i != i {mat[n-1-i][i]} else {0}
            )
            .sum()
    }
}