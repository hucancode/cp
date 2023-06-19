use std::cmp::max;
impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        max(left.into_iter()
            .max()
            .unwrap_or(0), 
            n - right.into_iter()
            .min()
            .unwrap_or(n))
    }
}