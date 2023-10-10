use std::cmp::max;
impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let m = nums2.len();
        let mut f = vec![vec![-1000_000;m+1];n+1];
        let mut ret = i32::MIN;
        for i in 1..=n {
            for j in 1..=m {
                let x = nums1[i-1]*nums2[j-1];
                f[i][j] = max(f[i][j], x);
                f[i][j] = max(f[i][j], f[i-1][j-1]);
                f[i][j] = max(f[i][j], f[i-1][j-1]+x);
                f[i][j] = max(f[i][j], f[i][j-1]);
                f[i][j] = max(f[i][j], f[i-1][j]);
                ret = max(ret, f[i][j]);
            }
        }
        return ret;
    }
}