impl Solution {
    pub fn min_difference(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 5 { return 0; }
        nums.sort();
        (0..4).map(|i| nums[nums.len() - 4 + i] - nums[i])
            .min()
            .unwrap()
    }
}
