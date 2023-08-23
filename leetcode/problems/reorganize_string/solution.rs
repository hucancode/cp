use std::collections::BinaryHeap;
use std::collections::HashMap;
impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut count = HashMap::new();
        for c in s.chars() {
            count.entry(c)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
        let mut pool: BinaryHeap<(usize, char)> = count.into_iter()
            .map(|(k,v)|(v,k))
            .collect();
        let mut ret = String::new();
        let mut cx = 0;
        let mut x = '.';
        while let Some((cy, y)) = pool.pop() {
            ret.push(y);
            if cx > 0 {
                pool.push((cx, x));
            }
            cx = cy - 1;
            x = y;
        }
        //println!("count {cx}, char = {x}");
        if cx > 0 {
            ret.clear();
        }
        return ret;
    }
}