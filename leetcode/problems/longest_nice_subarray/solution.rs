use std::collections::VecDeque;
use std::cmp::max;
impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let mut q = VecDeque::new();
        let mut ret = 0;
        let mut sum = 0;
        for x in nums {
            while let Some(y) = q.front() {
                if x & sum == 0 {
                    break;
                }
                sum &= !y;
                q.pop_front();
            }
            sum |= x;
            q.push_back(x);
            ret = max(ret, q.len());
        }
        ret as i32
    }
}