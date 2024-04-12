impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        use std::cmp::{min, max};
        let n = height.len();
        let mut left = vec![0;n];
        let mut right = vec![0;n];
        left[0] = height[0];
        right[n-1] = height[n-1];
        for i in 1..n {
            left[i] = max(left[i-1], height[i]);
        }
        for i in (0..n-1).rev() {
            right[i] = max(right[i+1], height[i]);
        }
        left.iter()
            .zip(right.iter())
            .zip(height.iter())
            .map(|((a,b),c)| min(a,b) - c)
            .sum()
    }
}
