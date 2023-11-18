use std::cmp::max;
impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let n = nums.len();
        let mut cost = 0;
        let mut j = 0;
        let mut ret = 1;
        for i in 1..n {
            cost += (nums[i] - nums[i-1]) * (i - j) as i32;
            while cost > k && j < i {
                cost -= nums[i] - nums[j];
                j += 1;
            }
            ret = max(ret, i - j + 1);
        }
        ret as _
    }
}