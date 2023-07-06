impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let n = nums.len();
        (0..n/2).map(|i| nums[i] + nums[n-i-1]).max().unwrap_or(0)
    }
}