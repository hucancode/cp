use std::cmp::Ordering;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let i = letters.binary_search_by(|c| match c.cmp(&target) {
            Ordering::Equal => Ordering::Less,
            ord => ord,
        })
        .unwrap_err();
        if i >= letters.len() {
            letters[0]
        } else {
            letters[i]
        }
    }
}