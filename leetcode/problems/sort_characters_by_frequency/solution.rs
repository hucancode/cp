use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut freq: HashMap<char, usize> = HashMap::new();
        let mut token: HashMap<usize, Vec<char>> = HashMap::new();
        for c in s.chars() {
            *freq.entry(c).or_default() += 1;    
        }
        for (c, f) in freq {
            token.entry(f).or_default().push(c);
        }
        let mut freq: Vec<usize> = token.keys().copied().collect();
        freq.sort();
        let mut ret = String::new();
        for f in freq.into_iter().rev() {
            for c in token.entry(f).or_default() {
                ret.extend(std::iter::repeat(*c).take(f))
            }
        }
        ret
    }
}