use std::cmp::Ordering;
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let i = nums
        .binary_search_by(|&x| {
            if x >= target {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        })
        .unwrap_err();
        let j = nums
        .binary_search_by(|&x| {
            if x <= target {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
        .unwrap_err();
        if i >= nums.len() || nums[i] != target {
            vec![-1,-1]
        } else {
            vec![i as i32, j as i32 - 1]
        }
    }
}