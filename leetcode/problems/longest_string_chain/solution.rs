use std::cmp::max;
impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let is_predecessor = |a: &[u8], b: &[u8]| {
            if a.len() + 1 != b.len() {
                return false;
            }
            let n = a.len();
            let mut l = 0;
            while l < n && a[l] == b[l] {
                l+= 1;
            }
            let mut r = 0;
            while r < n && a[n-1-r] == b[n-r] {
                r += 1;
            }
            return l+r >= n;
        };
        let mut g = vec![Vec::new();17];
        let mut f = vec![Vec::new();17];
        for word in words.iter() {
            let word = word.as_bytes();
            let n = word.len();
            if n > 16 {
                continue;
            }
            g[n].push(word);
            f[n].push(1);
        }
        let mut ret = 1;
        for len in 2..=16 {
            let n = g[len].len();
            let m = g[len-1].len();
            for i in 0..n {
                for j in 0..m {
                    if is_predecessor(g[len-1][j], g[len][i]) {
                        //println!("connect {:?}-{:?}", g[len-1][j], g[len][i]);
                        f[len][i] = max(f[len][i], f[len-1][j]+1);
                    }
                }
                ret = max(ret, f[len][i]);
            }
        }
        return ret;
    }
}