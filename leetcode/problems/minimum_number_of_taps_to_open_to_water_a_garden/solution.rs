use std::cmp::{min, max};
impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let INF = 1000_000_000;
        let mut f = vec![INF;n as usize+1];
        f[0] = 0;
        for (a,b) in ranges.iter()
            .enumerate()
            .map(|(i,len)| (i as i32 - len, i as i32 + len))
            .map(|(a,b)| (max(0, a) as usize, min(n,b) as usize))
         {
            for i in a+1..=b {
                f[i] = min(f[i], f[a]+1);
            }
        }
        let ret = f[n as usize];
        if ret == INF {-1} else {ret}
    }
}