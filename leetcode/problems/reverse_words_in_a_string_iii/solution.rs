impl Solution {
    pub fn reverse_words(s: String) -> String {
        // s.split_whitespace()
        //     .map(|s| s.chars().rev().collect::<String>())
        //     .intersperse(String::from(" "))
        //     .collect()
        s.split_whitespace()
            .map(|s| s.chars().rev().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ")
    }
}