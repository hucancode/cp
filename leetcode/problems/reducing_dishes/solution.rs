use std::cmp::max;

impl Solution {
    pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
        let mut a = satisfaction;
        a.sort();
        let n = a.len()+1;
        let mut f = vec![vec![0;n];n];
        for i in 1..n {
            f[i][i] = f[i-1][i-1] + a[i-1]*i as i32;
            //println!("at time {i}, want {i} disk, best score = {}", f[i][i]);
            for j in 1..i {
                let x = f[i-1][j-1] + a[i-1]*j as i32;
                let y = f[i-1][j];
                f[i][j] = max(x, y);
                //println!("at time {i}, want {j} disk, best score = {}", f[i][j]);
            }
        }
        if let Some(v) = f.last() {
            if let Some(ret) = v.iter().max() {
                return *ret;
            }
        }
        return 0;
    }
}