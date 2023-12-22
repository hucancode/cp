use std::collections::HashSet;
impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        let mut f = vec![0;1+n];
        let mut vis = HashSet::new();
        for i in 1..=n {
            f[i] = nums[i-1]+f[i-1];
        }
        for i in 2..=n {
            let y = f[i-2] % k;
            vis.insert(y);
            let x = f[i] % k;
            if vis.contains(&x) {
                return true;
            }
        }
        return false;
    }
}