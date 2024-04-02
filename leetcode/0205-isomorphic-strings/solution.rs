impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let check = |s: &[u8], t: &[u8]| {
            let mut st = std::collections::HashMap::new();
            s.iter().zip(t.iter()).all(|(&s, &t)| {
                match(st.insert(s,t)) {
                    Some(x) => x == t,
                    _ => true,
                }
            })
        };
        check(s, t) && check(t, s)
    }
}
