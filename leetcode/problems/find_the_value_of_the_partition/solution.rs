impl Solution {
    pub fn find_value_of_partition(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        nums.windows(2)
            .map(|a| a[1] - a[0])
            .min()
            .unwrap_or(0)
    }
}