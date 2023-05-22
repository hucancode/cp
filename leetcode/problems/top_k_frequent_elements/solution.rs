use std::collections::HashSet;
use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut bucket: Vec<HashSet<i32>> = Vec::new();
        let mut count: HashMap<i32,i32> = HashMap::new();
        for x in nums {
            let c = *count.entry(x)
                .and_modify(|c| *c += 1)
                .or_insert(1);
            let prev = c-1;
            if prev > 0 {
                bucket[prev as usize].remove(&x);
            }
            while c >= bucket.len() as i32 {
                bucket.push(HashSet::new());
            }
            bucket[c as usize].insert(x);
        }
        let mut ret = Vec::new();
        while k > 0 {
            if let Some(set) = bucket.pop() {
                k -= set.len() as i32;
                ret.extend(set.iter());
            } else {
                break;
            }
        }
        ret
    }
}