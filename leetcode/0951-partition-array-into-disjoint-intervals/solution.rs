impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max_prefix = vec![i32::MIN;n+1];
        let mut min_suffix = vec![i32::MAX;n+1];
        for i in 1..=n {
            max_prefix[i] = max_prefix[i-1].max(nums[i-1]);
        }
        for i in (0..n).rev() {
            min_suffix[i] = min_suffix[i+1].min(nums[i]);
        }
        for i in 1..n {
            if max_prefix[i] <= min_suffix[i] {
                return i as i32;
            }
        }
        return -1;
    }
}
