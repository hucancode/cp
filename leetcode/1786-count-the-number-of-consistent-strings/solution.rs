impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let allowed: std::collections::HashSet<_> = allowed.chars().collect();
        words.iter()
            .filter(|w| w.chars().all(|c| allowed.contains(&c)))
            .count() as i32
    }
}
