use std::cmp::min;
impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        let INF = 1000_000_007;
        let target = target as usize;
        let k = k as usize;
        let mut f = vec![0i64; 1 + target];
        f[0] = 1;
        for _ in 0..n {
            let mut next = vec![0i64; 1 + target];
            for x in 1..=target {
                for y in 1..=min(x,k) {
                    next[x] += f[x-y];
                    next[x] %= INF;
                }
            }
            f = next;
            //println!("({f:?})");
        }
        return f[target] as i32;
    }
}