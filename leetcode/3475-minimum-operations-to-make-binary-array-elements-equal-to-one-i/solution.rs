impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut f = vec![0;n];
        let mut ret = 0;
        for i in 0..n-2 {
            if (nums[i] + f[i])%2 == 0 {
                f[i] += 1;
                f[i+1] += 1;
                f[i+2] += 1;
                ret += 1;
            }
        }
        if (f[n-1] + nums[n-1])%2 == 1 && (f[n-2] + nums[n-2])%2 == 1 {
            return ret;
        }
        return -1;
    }
}
