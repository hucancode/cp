use std::collections::HashMap;
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut map = HashMap::new();
        for i in 0..n {
            let arr = grid[i].clone();
            map.entry(arr)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
        (0..n).map(|j| {
            let col: Vec<i32> = grid.iter()
                .map(|arr| arr[j])
                .collect();
            *map.get(&col).unwrap_or(&0)
        }).sum()
    }
}