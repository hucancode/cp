use std::cmp::min;
impl Solution {
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let target: Vec<usize> = target.as_bytes()
                .into_iter()
                .map(|&c| c as usize - 'a' as usize)
                .collect();
        let stickers: Vec<Vec<usize>> = stickers
            .into_iter()
            .map(|s| s.as_bytes()
                .into_iter()
                .map(|&c| c as usize - 'a' as usize)
                .collect())
            .collect();
        let n = target.len();
        let k = 1<<n;
        let mut f = vec![1 + n as i32;k];
        let mut indices = vec![vec![vec![];26];k];
        f[0] = 0;
        for (i, c) in target.iter().enumerate() {
            indices[0][*c].push(i);
        }
        for i in 0..k {
            for s in stickers.iter() {
                let mut next = i;
                let mut index = indices[i].clone();
                for c in s.iter() {
                    if let Some(j) = index[*c].pop() {
                        next |= 1<<j;
                    }
                }
                f[next] = min(f[next], f[i] + 1);
                indices[next] = index;
            }
        }
        if f[k-1] > n as i32 {
            f[k-1] = -1;
        }
        return f[k-1];
    }
}