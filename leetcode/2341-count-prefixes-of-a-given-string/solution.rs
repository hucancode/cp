impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        words.iter()
            .filter(|&w| s.starts_with(w))
            .count() as i32
    }
}
