impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut x = 0;
        (0..1<<n).map(|i| {
            x ^= (i & -i);
            x
        })
        .collect()
    }
}