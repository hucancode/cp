use std::collections::HashSet;
use std::cmp::min;
impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let pool: HashSet<String> = dictionary.into_iter().collect();
        let n = s.len();
        let mut f = vec![0;n+1];
        for i in 1..=n {
            f[i] = f[i-1]+1;
            for j in 1..=i {
                let sub = s.get(j-1..i).unwrap_or("");
                if !pool.contains(&sub.to_string()) {
                    continue;
                }
                for k in 0..j {
                    f[i] = min(f[i], f[k] + j-k-1);
                }
            }
        }
        (1..=n).map(|i| f[i] + n-i).min().unwrap_or(n) as i32
    }
}