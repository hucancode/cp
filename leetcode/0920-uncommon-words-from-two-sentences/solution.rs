impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        use std::collections::HashSet;
        let mut ret = HashSet::new();
        let mut dup = HashSet::new();
        for s in s1.split(' ').chain(s2.split(' ')) {
            if !ret.insert(s) {
                dup.insert(s);
            }
        }
        ret.difference(&dup)
            .map(|s| s.to_string())
            .collect()
    }
}
