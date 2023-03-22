impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        if(k == 0) {
            return 1.0;
        }
        let max_pts = max_pts as usize;
        let k = k as usize;
        let n = n as usize;
        let mut f = vec![1.0;max_pts+k];
        let mut sum = 1.0;
        for i in 1..f.len() {
            if(i > max_pts) {
                sum -= f[i-max_pts-1];
            }
            f[i] = sum/max_pts as f64;
            if(i < k) {
                sum += f[i];
            }
        }
        let x: f64 = f.iter().skip(n + 1).sum();
        return 1.0 - x;
    }
}