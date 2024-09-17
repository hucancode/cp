impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        use std::collections::HashSet;
        let mut ret1 = HashSet::new();
        let mut dup1 = HashSet::new();
        let mut ret2 = HashSet::new();
        let mut dup2 = HashSet::new();
        for s in words1.iter() {
            if !ret1.insert(s) {
                dup1.insert(s);
            }
        }
        let ret1: HashSet<&String> = ret1.difference(&dup1)
            .cloned()
            .collect();
        for s in words2.iter() {
            if !ret2.insert(s) {
                dup2.insert(s);
            }
        }
        let ret2: HashSet<&String> = ret2.difference(&dup2)
            .cloned()
            .collect();
        ret1.intersection(&ret2)
            .count() as i32
    }
}
