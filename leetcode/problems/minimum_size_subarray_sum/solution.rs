use std::cmp::Ordering;
use std::cmp::min;
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut prefix = vec![0;n+1];
        for i in 1..=n {
            prefix[i] = prefix[i-1] + nums[i-1];
        }
        let mut ret = n+1;
        for i in 1..=n {
            let x = prefix[i] - target;
            let j = prefix.binary_search_by(
                |&y| if y > x {Ordering::Greater} else {Ordering::Less})
            .unwrap_err();
            if j == 0 {
                continue;
            }
            ret = min(ret, i-j+1);
        }
        if ret > n {
            ret = 0;
        }
        return ret as i32;
    }
}