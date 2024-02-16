use std::collections::HashMap;
impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, mut k: i32) -> i32 {
        let mut freq: HashMap<i32, i32> = HashMap::new();
        for x in arr {
            *freq.entry(x).or_default() += 1;
        }
        let mut freq: Vec<i32> = freq.values().cloned().collect();
        freq.sort();
        let n = freq.len();
        for (i, f) in freq.into_iter().enumerate() {
            if k < f {
                return (n-i) as i32
            }
            k -= f;
        }
        return 0;
    }
}