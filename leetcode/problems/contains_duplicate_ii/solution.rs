use std::collections::HashSet;
use std::cmp::min;
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut set = HashSet::new();
        for i in 0..nums.len() {
            if i > k {
                set.remove(&nums[i-k-1]);
            }
            set.insert(nums[i]);
            if set.len() <= min(i,k) {
                return true;
            }
        }
        return false;
    }
}