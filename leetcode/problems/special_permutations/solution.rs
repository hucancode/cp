const INF: i32 = 1000_000_007;
impl Solution {
    pub fn special_perm(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = 1 << n;
        let mut f = vec![vec![0;n];m];
        for i in 0..n {
            f[1<<i][i] = 1;
        }
        for mask in 1..m {
            for i in (0..n)
            .filter(|i| mask & (1<<i) != 0) {
                let maski = mask & !(1<<i);
                for j in (0..n)
                .filter(|&j| maski & (1<<j) != 0)
                .filter(|&j| nums[i] % nums[j] == 0 || nums[j] % nums[i] == 0) {
                    f[mask][i] += f[maski][j];
                    f[mask][i] %= INF;
                    //println!("take {} then {}", nums[i], nums[j]);
                }
                //println!("state {mask:#6b}, take {}, score = {}", nums[i], f[mask][i]);
            }
        }
        f[m-1].iter().fold(0, |acc, x| (acc + x)%INF)
    }
}