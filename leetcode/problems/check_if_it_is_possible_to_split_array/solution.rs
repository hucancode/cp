impl Solution {
    pub fn can_split_array(nums: Vec<i32>, m: i32) -> bool {
        nums.len() <= 2 || nums.windows(2).any(|a| a[0] + a[1] >= m)
    }
}