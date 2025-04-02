impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut max_prefix = vec![0; n+1];
        let mut max_suffix = vec![0; n+1];
        let mut max_ij_prefix = vec![0; n+1];
        for i in 1..=n {
            max_prefix[i] = max_prefix[i-1].max(nums[i-1]);
        }
        for i in (0..n).rev() {
            max_suffix[i] = max_suffix[i+1].max(nums[i]);
        }
        (1..n-1)
            .filter(|&j| max_prefix[j] > nums[j])
            .map(|j| (max_prefix[j] - nums[j]) as i64 * max_suffix[j+1] as i64)
            .max()
            .unwrap_or(0)
    }
}
