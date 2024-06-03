impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let mut i = s.chars();
        t.chars()
            .skip_while(|a| i.find(|b| b == a).is_some())
            .count() as i32
    }
}
