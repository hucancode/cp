use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut f: HashMap<Vec<u8>,Vec<String>> = HashMap::new();
        for s in strs {
            let mut k = vec![0;26];
            for c in s.chars() {
                k[c as usize - 'a' as usize] += 1;
            }
            f.entry(k).or_default().push(s);
        }
        f.into_values().collect()
    }
}