use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::max;
impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let mut tails = HashSet::new();
        let mut f = HashMap::new();
        for x in nums {
            for &y in tails.iter() {
                let prev = (y, x-y);
                let current = (x, x-y);
                if f.contains_key(&prev) {
                    let d1 = f[&prev];
                    f.entry(current)
                        .and_modify(|d2| *d2 = max(*d2, d1+1))
                        .or_insert(d1+1);
                } else {
                    f.insert(current, 2);
                }
            }
            f.entry((x, 0))
                .or_insert(1);
            tails.insert(x);
        }
        f.into_iter()
            .map(|(k,v)| v)
            .max()
            .unwrap_or(0) as i32
    }
}