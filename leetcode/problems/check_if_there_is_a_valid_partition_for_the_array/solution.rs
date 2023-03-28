impl Solution {
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        let n = nums.len() + 1;
        let mut f = vec![false; n];
        f[0] = true;
        for i in 2..n {
            if nums[i-1] == nums[i-2] {
                f[i] = f[i] || f[i-2];
            }
            if i < 3 {
                continue;
            }
            if nums[i-1] == nums[i-2] && 
                nums[i-1] == nums[i-3] {
                f[i] = f[i] || f[i-3];
            }
            if nums[i-1] == nums[i-2]+1 && 
                nums[i-1] == nums[i-3]+2 {
                f[i] = f[i] || f[i-3];
            }
        }
        return f[n-1];
    }
}