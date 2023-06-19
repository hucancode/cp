use std::cmp::max;
impl Solution {
    pub fn longest_semi_repetitive_substring(s: String) -> i32 {
        let n = s.len();
        if n <= 2 {
            return n as i32;
        }
        let s = s.as_bytes();
        let mut ret = 0;
        for i in 0..n {
            for j in i+1..n {
                let mut pair = 0;
                for k in i+1..=j {
                    if s[k] == s[k-1] {
                        pair += 1;
                    }
                }
                if pair < 2 {
                    ret = max(ret, j-i+1);
                }
            }
        }
        ret as i32
    }
}