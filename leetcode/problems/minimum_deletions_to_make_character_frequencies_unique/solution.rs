use std::cmp::max;
use std::cmp::min;
impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let s = s.as_bytes();
        let n = 26;
        let mut freq = vec![0;n];
        for &c in s {
            let i = c as usize - 'a' as usize;
            freq[i] += 1;
        }
        freq.sort_by(|a,b|b.cmp(&a));
        let mut ret = 0;
        let mut top = i32::MAX;
        for x in freq {
            let target = max(0, top - 1);
            ret += max(0, x - target);
            top = min(x, target);
        }
        return ret;
    }
}