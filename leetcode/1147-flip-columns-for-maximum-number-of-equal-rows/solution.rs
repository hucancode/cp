impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let n = matrix.len();
        let m = matrix[0].len();
        let mut occ = HashMap::new();
        for row in matrix {
            let x = row[0];
            let f: Vec<bool> = row.into_iter()
                .map(|y| x == y)
                .collect();
            *occ.entry(f).or_default() += 1;
        }
        *occ.iter()
            .map(|(k,v)| v)
            .max()
            .unwrap_or(&0)
    }
}
