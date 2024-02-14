use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn match_replacement(s: String, sub: String, mappings: Vec<Vec<char>>) -> bool {
        let mut f = vec![vec![false;256];256];
        for a in mappings {
            let k = a[0] as usize;
            let v = a[1] as usize;
            f[k][v] = true;
        }
        let s = s.as_bytes();
        let sub = sub.as_bytes();
        let n = s.len();
        let m = sub.len();
        (0..=(n-m)).any(|i| 
            (0..m).all(|j| 
                sub[j] == s[i+j] || f[sub[j] as usize][s[i+j] as usize]
            )
        )
    }
}