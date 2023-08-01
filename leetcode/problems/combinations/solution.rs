impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        (1..1<<n)
            .filter(|x: &i32| x.count_ones() == k as u32)
            .map(|mask| {
                (0..n)
                    .filter(|i| mask & 1<<i != 0)
                    .map(|i| i+1)
                    .collect()
            })
            .collect()
    }
}