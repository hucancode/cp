impl Solution {
    pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
        happiness.sort();
        happiness.into_iter()
            .rev()
            .enumerate()
            .map(|(i,x)| (x - i as i32) as i64)
            .take(k as usize)
            .take_while(|&x| x > 0)
            .sum()
    }
}
