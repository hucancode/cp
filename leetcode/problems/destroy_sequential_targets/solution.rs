use std::collections::HashMap;
use std::cmp::min;
impl Solution {
    pub fn destroy_targets(mut nums: Vec<i32>, space: i32) -> i32 {
        let mut len = HashMap::new();
        let mut head = HashMap::new();
        for x in nums {
            let y = x % space;
            len.entry(y)
                .and_modify(|l| *l += 1)
                .or_insert(1);
            head.entry(y)
                .and_modify(|z| *z = min(*z, x))
                .or_insert(x);
        }
        let (k, v) = len.into_iter()
            .reduce(|(k1,v1), (k2,v2)| {
                if v1 > v2 {
                    return (k1, v1);
                }
                if v2 > v1 {
                    return (k2, v2);
                }
                if head.get(&k1).unwrap_or(&0) < head.get(&k2).unwrap_or(&0) {
                    return (k1, v1);
                }
                return (k2, v2);
            })
            .unwrap_or((0,0));
        *head.get(&k).unwrap_or(&0)
    }
}