use std::cmp::max;
impl Solution {
    pub fn max_score(s: String) -> i32 {
        let s: Vec<usize> = s.chars()
            .map(|c| if c == '1' {1} else {0} )
            .collect();
        let n = s.len();
        let mut p = vec![0;n+1];
        for i in 1..=n {
            p[i] = p[i-1] + s[i-1];
        }
        let mut ret = 0;
        for i in 1..n {
            let a = i - p[i];
            let b = p[n] - p[i];
            ret = max(ret, a+b);
        }
        return ret as i32;
    }
}