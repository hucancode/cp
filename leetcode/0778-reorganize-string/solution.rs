impl Solution {
    pub fn reorganize_string(s: String) -> String {
        use std::collections::{BinaryHeap, HashMap};
        let mut counts = HashMap::new();
        for c in s.chars() {
            counts.entry(c)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
        let mut candidates: BinaryHeap<(i32, char)> = counts.into_iter()
            .map(|(k,v)| (v,k))
            .collect();
        //println!("{candidates:?}");
        let mut last = '.';
        let mut last_count = -1;
        let mut ret = String::new();
        while let Some((count, c)) = candidates.pop() {
            if last_count > 0 {
                candidates.push((last_count, last));
            }
            last = c;
            last_count = count-1;
            ret.push(c);
        }
        if last_count != 0 {
            return String::new();
        }
        return ret;
    }
}
