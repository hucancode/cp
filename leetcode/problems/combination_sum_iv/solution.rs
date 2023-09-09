impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let n = target as usize;
        let mut f = vec![0;n+1];
        f[0] = 1;
        for i in 1..=n {
            for &j in nums.iter() {
                let j = j as usize;
                if j <= i {
                    f[i] += f[i - j];
                }
            }
        }
        f[n]
    }
}