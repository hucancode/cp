use std::collections::HashMap;
impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map: HashMap<usize, Vec<i32>> = HashMap::new();
        for (i, &n) in group_sizes.iter().enumerate() {
            map.entry(n as usize)
                .or_default()
                .push(i as i32);
        }
        map.into_iter()
            .map(|(n, arr)| arr.chunks(n)
                .map(|arr| arr.to_vec())
                .collect::<Vec<Vec<i32>>>())
            .flatten()
            .collect()
    }
}