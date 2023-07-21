use std::collections::HashMap;
impl Solution {
    pub fn relocate_marbles(nums: Vec<i32>, move_from: Vec<i32>, move_to: Vec<i32>) -> Vec<i32> {
        let mut occupied = HashMap::new();
        for x in nums {
            occupied.insert(x, true);
        }
        for (a,b) in move_from.into_iter().zip(move_to.into_iter()) {
            if let Some(&v) = occupied.get(&a) {
                if v {
                    occupied.insert(a, false);
                    occupied.insert(b, true);
                }
            }
        }
        let mut ret: Vec<i32> = occupied.into_iter()
            .filter_map(|(k,v)| if v { Some(k) } else { None })
            .collect();
        ret.sort();
        return ret;
    }
}