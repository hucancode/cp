use std::cmp::max;
use std::collections::HashSet;
impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut set = HashSet::new();
        let mut score = 0;
        let mut l = 0;
        let mut ret = 0;
        for r in 0..n {
            while set.contains(&nums[r]) {
                set.remove(&nums[l]);
                score -= nums[l];
                l += 1;
            }
            set.insert(nums[r]);
            score += nums[r];
            ret = max(ret, score);
        }
        ret
    }
}