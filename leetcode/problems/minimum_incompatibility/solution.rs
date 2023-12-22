use std::cmp::{min, max};
impl Solution {
    pub fn minimum_incompatibility(nums: Vec<i32>, k: i32) -> i32 {
        let INF = 1000000;
        let n = nums.len();
        let k = k as usize;
        let m = 1<<n;
        let mut f = vec![-1;m];
        f[0] = 0;
        let len = n/k;
        let mut unit: Vec<(usize, i32)> = (0..m)
            .filter_map(|mask| {
                if mask.count_ones() != len as u32 {
                    return None;
                }
                let mut picked = 0;
                let mut a = i32::MIN;
                let mut b = i32::MAX;
                for i in (0..n).filter(|i| mask & (1<<i) != 0) {
                    if picked & 1<<nums[i] != 0 {
                        return None;
                    }
                    picked |= 1<<nums[i];
                    a = max(a, nums[i]);
                    b = min(b, nums[i]);
                }
                Some((mask, a - b))
            })
            .collect();
        let mut q = vec![0];
        let mut f = vec![INF;m];
        let mut vis = vec![false;m];
        f[0] = 0;
        while !q.is_empty() {
            let mut qnext = Vec::new();
            while let Some(mask) = q.pop() {
                if vis[mask] {
                    continue;
                }
                vis[mask] = true;
                if f[mask] >= INF {
                    continue;
                }
                for &(x, score) in unit.iter()
                    .filter(|&(x, _)| (x & mask) == 0) {
                    let next = x | mask;
                    f[next] = min(f[next], score + f[mask]);
                    qnext.push(next);
                }
            }
            q = qnext;
        }
        if(f[m-1] < INF) {
            f[m-1]
        } else {
            -1
        }
    }
}