use std::cmp::max;
use std::collections::HashMap;
impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let mut f: HashMap<i32, i32> = HashMap::new();
        for x in arr {
            let target = x - difference;
            let mut next = 1;
            if let Some(count) = f.get(&target) {
                next = count + 1;
            }
            f.entry(x)
                .and_modify(|count| *count = max(*count, next))
                .or_insert(next);
        }
        f.into_iter()
            .map(|(k,v)| v)
            .max()
            .unwrap_or(1)
    }
}