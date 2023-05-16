use std::cmp::max;

impl Solution {
    pub fn find_max_consecutive_ones(mut nums: Vec<i32>) -> i32 {
        nums.iter()
            .scan(0, |acc, &x| {
                if x == 0 {*acc = 0} else {*acc += 1};
                Some(*acc)
            })
            .max()
            .unwrap_or(0) as i32
    }
}