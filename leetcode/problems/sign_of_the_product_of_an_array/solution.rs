impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        nums.iter().fold(1, |a,b| (a * b).signum())
    }
}