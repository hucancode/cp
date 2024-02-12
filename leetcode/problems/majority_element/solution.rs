impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut ret = nums[0];
        let mut freq = 0;
        for x in nums {
            if freq == 0 {
                ret = x;
            }
            freq += if ret == x { 1 } else { -1 };
        }
        return ret;
    }
}