impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut min_prefix = vec![1000_000_000; n+1];
        let mut min_suffix = vec![1000_000_000; n+1];
        for i in 1..=n {
            min_prefix[i] = min_prefix[i-1].min(nums[i-1]);
        }
        for i in (0..n).rev() {
            min_suffix[i] = min_suffix[i+1].min(nums[i]);
        }
        println!("{min_prefix:?} {min_suffix:?}");
        (1..n-1)
            .filter(|&j| nums[j] > min_prefix[j])
            .filter(|&j| nums[j] > min_suffix[j+1])
            .map(|j| min_prefix[j] + nums[j] + min_suffix[j+1])
            .min()
            .unwrap_or(-1)
    }
}
