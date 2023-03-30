use std::cmp::min;
use std::cmp::max;
use std::collections::HashMap;
impl Solution {
    pub fn solve(s1: &str, s2: &str, f: &mut HashMap<(String, String), bool>) -> bool {
        let hash_key = (s1.to_string(), s2.to_string());
        if let Some(&ret) = f.get(&hash_key) {
            return ret;
        }
        let mut ret = false;
        if(s1 == s2) {
            ret = true;
            f.insert(hash_key, ret);
            return ret;
        }
        let n = s1.len();
        for i in 1..n {
            let j = n-i;
            let a1 = &s1[0..i];
            let b1 = &s1[i..n];
            let a2 = &s2[0..i];
            let b2 = &s2[i..n];
            let c2 = &s2[0..j];
            let d2 = &s2[j..n];
            if Self::solve(a1, a2, f) && Self::solve(b1, b2, f) {
                ret = true;
                break;
            }
            if Self::solve(a1, d2, f) && Self::solve(b1, c2, f) {
                ret = true;
                break;
            }
        }
        f.insert(hash_key, ret);
        return ret;
    }
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let mut f = HashMap::new();
        return Self::solve(&s1, &s2, &mut f);
    }
}