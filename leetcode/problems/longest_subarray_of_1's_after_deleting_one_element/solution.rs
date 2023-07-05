use std::cmp::max;
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        if nums.iter().all(|&x| x == 1) {
            return nums.len() as i32 - 1;
        }
        let mut x = 0;
        let mut y = 0;
        let mut ret = 0;
        for i in nums {
            if i == 1 {
                y += 1;
            } else {
                ret = max(ret, x+y);
                x = y;
                y = 0;
            }
        }
        ret = max(ret, x+y);
        return ret;
    }
}