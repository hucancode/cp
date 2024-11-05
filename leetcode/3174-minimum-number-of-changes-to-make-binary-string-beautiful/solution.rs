impl Solution {
    pub fn min_changes(s: String) -> i32 {
        s.as_bytes()
            .chunks(2)
            .filter(|a| a[0] != a[1])
            .count() as i32
    }
}
