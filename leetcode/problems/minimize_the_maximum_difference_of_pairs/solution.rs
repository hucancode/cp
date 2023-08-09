use std::cmp::max;
use std::cmp::min;
impl Solution {
    pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
        nums.sort();
        if p == 0 {
            return 0;
        }
        let n = nums.len();
        let check = |x| {
            let mut ret = 0;
            let mut i = 1;
            while i < n {
                if nums[i] - nums[i-1] <= x {
                    ret += 1;
                    if ret >= p {
                        return true;
                    }
                    i += 1;
                }
                i += 1;
            }
            return false;
        };
        let mut l = 0;
        let mut r = 1000_000_000;
        while l < r {
            let m = (l+r)/2;
            if check(m) {
                r = m;
            } else {
                l = m+1;
            }
        }
        return l;
    }
}