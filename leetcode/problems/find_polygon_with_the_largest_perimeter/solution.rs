use std::cmp::max;
impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
        nums.sort();
        let mut ret = -1;
        let mut sum = 0;
        for (i,x) in nums.into_iter().enumerate() {
            if sum > x as i64 {
                ret = max(ret, sum + x as i64)
            }
            sum += x as i64;
        }
        ret
    }
}