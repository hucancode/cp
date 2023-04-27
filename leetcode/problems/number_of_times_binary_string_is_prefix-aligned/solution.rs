use std::cmp::max;

impl Solution {
    pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
        let n = flips.len();
        let mut k = 0usize;
        let mut ret = 0;
        for i in 0..n {
            k = max(k, flips[i] as usize);
            if k == i + 1 {
                ret += 1;
            }
        }
        return ret;
    }
}