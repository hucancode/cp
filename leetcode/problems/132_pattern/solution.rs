use std::cmp::{min, max};
use std::collections::VecDeque;
impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n < 3 {
            return false;
        }
        let mut f1 = vec![i32::MAX; n];
        f1[0] = nums[0];
        for i in 1..n {
            f1[i] = min(f1[i-1], nums[i]);
        }
        let mut f3 = vec![1];
        for k in 2..n {
            let j = k-1;
            while let Some(&prev) = f3.last() {
                if nums[j] >= nums[prev] {
                    f3.pop();
                } else {
                    break;
                }
            }
            f3.push(j);
            for j in 0..f3.partition_point(|&j| nums[j] > nums[k]) {
                let j = f3[j];
                //println!("check f3 = {}, f1 = {} f2 = {}", nums[j], f1[j-1], nums[k]);
                if(f1[j-1] < nums[k] && nums[k] < nums[j]) {
                    return true;
                }
            }
        }
        return false;
    }
}