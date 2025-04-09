impl Solution {
    pub fn min_operations(mut nums: Vec<i32>, mut k: i32) -> i32 {
        nums.sort();
        if k > nums[0] {
            return -1;
        }
        let mut ret = 0;
        for x in nums {
            if x > k {
                ret += 1;
                k = x;
            }
        }
        return ret;
    }
}
