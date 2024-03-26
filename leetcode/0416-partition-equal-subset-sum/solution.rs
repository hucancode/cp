use std::cmp::min;
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let n: i32 = nums.iter().sum();
        if n&1 != 0 {
            return false;
        }
        let n = (n/2) as usize;
        let mut f = vec![false;n+1];
        f[0] = true;
        let mut cap = 0;
        for x in nums {
            let x = x as usize;
            cap = min(cap + x, n);
            let mut i = cap;
            while i >= x {
                f[i] |= f[i-x];
                i -= 1;
            }
        }
        return f[n];
    }
}
