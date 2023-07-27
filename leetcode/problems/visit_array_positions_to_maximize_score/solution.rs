use std::cmp::max;
impl Solution {
    pub fn max_score(nums: Vec<i32>, x: i32) -> i64 {
        let n = nums.len();
        let mut even = -1000_000_000_000;
        let mut odd = -1000_000_000_000;
        if nums[0]%2 == 0 {
            even = nums[0] as i64;
        } else {
            odd = nums[0] as i64;
        }
        let mut ret = nums[0] as i64;
        for i in 1..n {
            if nums[i]%2 == 0 {
                let a = even + nums[i] as i64;
                let b = odd + nums[i] as i64 - x as i64;
                even = max(a, b);
            } else {
                let a = even + nums[i] as i64 - x as i64;
                let b = odd + nums[i] as i64;
                odd = max(a, b);
            }
            ret = max(ret, max(even, odd));
        }
        return ret;
    }
}