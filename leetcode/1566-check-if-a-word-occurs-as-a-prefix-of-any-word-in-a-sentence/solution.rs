impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        sentence.split_whitespace()
            .position(|s| s.starts_with(&search_word))
            .map_or(-1, |i| 1+i as i32)
    }
}
