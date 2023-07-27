impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        words.into_iter()
            .map(|w| w.split(separator)
                    .map(|s| String::from(s))
                    .filter(|w| !w.is_empty())
                    .collect::<Vec<String>>()
            )
            .flatten()
            .collect()
    }
}