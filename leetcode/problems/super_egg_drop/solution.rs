use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;

impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let k = k as usize;
        let n = n as usize;
        // f[i][j] = the best moves count where we have j eggs, need to test i floors
        let mut f = vec![vec![10000;k+1];n+1];
        f[0].fill(0);
        for i in 1..=n {
            f[i][1] = i as i32;
            for egg in 2..=k {
                // have `egg` eggs, need to test `i` floors
                // find a middle floor `m` to drop, where the worst outcome is minimized
                // in other words, find floor `m` where live/die score is the most balanced
                let mut l = 0;
                let mut r = i;
                while l < r {
                    let m = (l+r)/2;
                    let live = f[m][egg];
                    let die = f[i-m-1][egg-1];
                    //println!("range - {l}-{r}({m}), s {live} d {die}");
                    f[i][egg] = min(f[i][egg], 1+max(live, die));
                    if live > die {
                        r = m;
                    } else {
                        l = m+1;
                    }
                }
            }
        }
        return f[n][k];
    }
}