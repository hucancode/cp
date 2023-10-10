use std::cmp::{Ordering, min};
impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.sort();
        //println!("{nums:?}");
        let mut ret = n-1;
        nums.dedup();
        for (i,l) in nums.iter().enumerate() {
            let r = l + n as i32 - 1;
            let mut j = nums
                .binary_search_by(|&x| {
                    if x >= r {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                })
                .unwrap_err();
            if j < nums.len() && nums[j] == r {
                j += 1;
            }
            let mut cost = i+n-j;
            //println!("use {l} as min, {r} as max, can reuse [{i},{j}), cost = {cost}");
            ret = min(ret, cost);
        }
        return ret as i32;
    }
}