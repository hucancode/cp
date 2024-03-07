use std::cmp::{min, max};
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut ret = 0;
        while l < r {
            ret = max(ret, min(height[r], height[l])*(r-l) as i32);
            if height[r] > height[l] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        return ret;
    }
}
