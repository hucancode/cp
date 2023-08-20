impl Solution {
    pub fn is_acronym(words: Vec<String>, s: String) -> bool {
        words.iter().map(|s|s.chars().take(1).last().unwrap()).collect::<String>() == s
    }
}