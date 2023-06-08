impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        grid.into_iter()
            .map(|arr| arr.into_iter().filter(|&x| x<0).count() as i32)
            .sum::<i32>()
    }
}