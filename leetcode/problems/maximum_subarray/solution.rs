use std::cmp::max;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = nums[0];
        let mut ret = sum;
        for x in nums.iter().skip(1) {
            sum = max(0, sum) + x;
            ret = max(ret, sum);
        }
        return ret;
    }
}