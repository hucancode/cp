use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let check = |s: &[u8], t: &[u8]| {
            let mut st = HashMap::new();
            for (s,t) in s.iter().zip(t.iter()) {
                match(st.insert(*s,*t)) {
                    Some(x) => if x != *t {return false},
                    _ => {},
                }
            }
            return true;
        };
        return check(&s, &t) && check(&t, &s);
    }
}