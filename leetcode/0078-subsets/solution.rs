impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        (0..1<<n)
            .map(|mask| (0..n)
                .filter(|i| mask & (1<<i) != 0)
                .map(|i| nums[i])
                .collect())
            .collect()
    }
}
