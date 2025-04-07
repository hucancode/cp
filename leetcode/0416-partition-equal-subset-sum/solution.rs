use std::cmp::min;
impl Solution {
    pub fn can_partition(mut nums: Vec<i32>) -> bool {
        let n: i32 = nums.iter().sum();
        if n%2 != 0 {
            return false;
        }
        let n = (n/2) as usize;
        let mut f = vec![false;n+1];
        f[0] = true;
        let mut cap = 0;
        for x in nums {
            let x = x as usize;
            cap = (cap+x).min(n);
            for i in (x..=cap).rev() {
                f[i] |= f[i-x];
            }
        }
        return f[n];
    }
}
