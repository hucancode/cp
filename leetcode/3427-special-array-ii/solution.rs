impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = nums.len();
        let mut valid_count = vec![0;n];
        for i in 1..n {
            valid_count[i] = valid_count[i-1] + if nums[i]%2 != nums[i-1]%2 {1} else {0};
        }
        queries.iter()
            .map(|v| (v[0] as usize, v[1] as usize))
            .map(|(i,j)|  valid_count[j] - valid_count[i] == j - i)
            .collect()
    }
}
