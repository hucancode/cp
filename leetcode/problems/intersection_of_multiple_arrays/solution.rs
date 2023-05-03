use std::collections::HashSet;
impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let nums = nums.into_iter()
            .map(|arr| arr.into_iter()
                .collect::<HashSet<i32>>())
            .collect::<Vec<HashSet<i32>>>();
        (1..=1000)
            .filter(|x| nums.iter()
                .all(|set| set.contains(x)))
            .collect::<Vec<i32>>()
    }
}