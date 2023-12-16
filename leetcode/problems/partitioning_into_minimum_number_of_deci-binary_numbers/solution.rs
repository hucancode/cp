impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        n.chars()
            .map(|c| c as i32 - '0' as i32)
            .max()
            .unwrap_or(0)
    }
}