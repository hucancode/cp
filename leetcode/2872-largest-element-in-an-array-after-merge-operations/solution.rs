impl Solution {
    pub fn max_array_value(nums: Vec<i32>) -> i64 {
        use std::cmp::max;
        let mut max_sum = 0;
        let mut sum = 0;
        for x in nums.into_iter().rev().map(|x| x as i64) {
            if x > sum {
                max_sum = max(max_sum, sum);
                sum = x;
            } else {
                sum += x;
            }
        }
        return max(max_sum, sum);
    }
}
