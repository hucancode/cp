use std::mem::swap;
use std::cmp::max;
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut a = 0;
        let mut b = 0;
        for mut x in nums {
            if x > a {
                swap(&mut x,&mut a);
            }
            b = max(x,b);
        }
        (a - 1) * (b - 1)
    }
}