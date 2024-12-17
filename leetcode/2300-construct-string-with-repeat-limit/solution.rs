impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        use std::collections::{HashMap, BinaryHeap};
        let mut count: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            *count.entry(c).or_default() += 1;
        }
        let mut q: BinaryHeap<(char, i32)> = count.into_iter().collect();
        let mut ret = String::new();
        let mut last_c = '.';
        let mut last_count = 0;
        while let Some((c, count)) = q.pop() {
            if last_count > 0 {
                q.push((last_c, last_count));
            }
            let take = if last_count == 0 || last_c < c { count.min(repeat_limit) } else { 1 };
            for _ in 0..take {
                ret.push(c);
            }
            last_count = count - take;
            last_c = c;
        }
        ret
    }
}
