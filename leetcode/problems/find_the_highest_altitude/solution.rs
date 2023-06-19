use std::cmp::max;
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut ret = 0;
        let mut x = 0;
        for y in gain {
            x += y;
            ret = max(x, ret);
        }
        ret
    }
}