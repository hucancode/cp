impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        let n = nums.len();
        let mut f = vec![0;n];
        let mut g = vec![0;n];
        for i in 0..n {
            for j in (0..i).rev() {
                if nums[i] > nums[j] {
                    f[i] = max(f[i], f[j]+1);
                }
            }
        }
        for i in (0..n).rev() {
            for j in i+1..n {
                if nums[i] > nums[j] {
                    g[i] = max(g[i], g[j]+1);
                }
            }
        }
        //println!("{f:?}, {g:?}");
        (1..n-1)
            .filter(|&i| f[i] != 0 && g[i] != 0)
            .map(|i| n - (f[i] + g[i] + 1))
            .min()
            .unwrap_or(n-1) as i32
    }
}
