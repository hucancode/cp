impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let n = nums.len();
        let mut counts: HashMap<i32, usize> = HashMap::new();
        for &x in nums.iter() {
            *counts.entry(x).or_default() += 1;
        }
        let mut occ: HashMap<i32, usize> = HashMap::new();
        for (i, &x) in nums.iter().enumerate() {
            if i >= n-1 {
                break;
            }
            *occ.entry(x).or_default() += 1;
            let k1 = *occ.entry(x).or_default();
            let k2 = *counts.entry(x).or_default() - k1;
            if k1 > (i+1)/2 && k2 > (n-i-1)/2 {
                return i as i32;
            }
        }
        return -1;
    }
}
