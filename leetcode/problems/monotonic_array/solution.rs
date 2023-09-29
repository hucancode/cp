impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let n = nums.len();
        nums.windows(2).all(|a| (a[0] == a[1]) || (a[0] > a[1]) == (nums[0] > nums[n-1]))
    }
}