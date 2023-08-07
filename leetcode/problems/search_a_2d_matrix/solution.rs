use std::cmp::Ordering;
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let r = matrix.binary_search_by(|row| {
            if row[0] <= target {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
        .unwrap_err();
        r > 0 && matrix[r-1].binary_search(&target).is_ok()
    }
}