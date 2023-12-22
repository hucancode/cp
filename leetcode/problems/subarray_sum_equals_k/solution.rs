use std::collections::HashMap;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut f = vec![0;1+n];
        for i in 1..=n {
            f[i] = f[i-1]+nums[i-1];
        }
        let mut vis = HashMap::new();
        vis.insert(0, 1);
        let mut ret = 0;
        for i in 1..=n {
            let target = f[i] - k;
            if let Some(&x) = vis.get(&target) {
                ret += x;
            }
            vis.entry(f[i])
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
        return ret;
    }
}