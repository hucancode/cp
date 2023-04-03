use std::collections::VecDeque;
use std::collections::HashMap;
use std::cmp::min;
use std::cmp::max;
impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let n = ring.len();
        let m = key.len();
        let INF = 100000;
        let mut f = vec![vec![INF;n];m+1];
        f[0][0] = 0;
        for i in 1..=m {
            let target = key.chars().nth(i-1).unwrap();
            for j in 0..n {
                let c = ring.chars().nth(j).unwrap();
                if c != target {
                    continue;
                }
                for k in 0..n {
                    let distance = (j as i32 - k as i32).abs();
                    let distance = min(distance, n as i32 - distance);
                    let score = f[i-1][k] + distance + 1;
                    f[i][j] = min(f[i][j], score);
                }
            }
        }
        return *f.last().unwrap().iter().min().unwrap();
    }
}