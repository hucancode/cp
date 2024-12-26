impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let INF: i32 = nums.iter().sum();
        let m = (INF*2+2) as usize;
        if target + INF < 0 || target + INF >= m as i32 {
            return 0;
        }
        let mut f = vec![vec![0;m];n+1];
        f[0][INF as usize] = 1;
        for i in 1..=n {
            for j in 0..m {
                let sum = j as i32 - INF as i32;
                let target = sum - nums[i-1];
                let k = target + INF as i32;
                if k >= 0 && k < m as i32 {
                    f[i][j] += f[i-1][k as usize];
                }
                let target = sum + nums[i-1];
                let k = target + INF as i32;
                if k >= 0 && k < m as i32 {
                    f[i][j] += f[i-1][k as usize];
                }
            }
        }
        //println!("{f:?}");
        f[n][(target + INF) as usize]
    }
}
