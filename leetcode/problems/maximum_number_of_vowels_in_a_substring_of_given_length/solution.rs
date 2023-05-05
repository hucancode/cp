use std::collections::VecDeque;
use std::collections::HashSet;
use std::cmp::max;
impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let k = k as usize;
        let vowels = HashSet::from(['a','e','i','o','u']);
        let mut q: VecDeque<usize> = VecDeque::new();
        let mut ret = 0;
        for (i, c) in s.chars().enumerate() {
            if vowels.contains(&c) {
                q.push_back(i);
            }
            if let Some(&j) = q.front() {
                if i - j >= k {
                    q.pop_front();
                }
            }
            ret = max(q.len() as i32, ret);
        }
        ret
    }
}