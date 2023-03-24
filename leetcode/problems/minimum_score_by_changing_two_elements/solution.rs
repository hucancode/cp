impl Solution {
    pub fn minimize_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 4 {
            return 0;
        }
        let mut nums = nums;
        nums.sort();
        let candidates = vec![
            nums[n-3] - nums[0],
            nums[n-2] - nums[1],
            nums[n-1] - nums[2],
        ];
        return *candidates.iter().min().unwrap_or(&0);
    }
}