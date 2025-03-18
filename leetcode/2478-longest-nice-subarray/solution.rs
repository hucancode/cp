impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let mut ret = 0;
        let mut j = 0;
        let mut sum = 0;
        for i in 0..nums.len() {
            while nums[i] & sum != 0 {
                sum &= !nums[j];
                j += 1;
            }
            ret = ret.max(i-j+1);
            sum |= nums[i];
        }
        return ret as i32;
    }
}
