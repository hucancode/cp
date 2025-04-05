impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.into_iter().fold(0, |acc, x| acc|x) << (n - 1)
    }
}
