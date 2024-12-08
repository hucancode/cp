impl Solution {
    pub fn max_value(mut events: Vec<Vec<i32>>, mut k: i32) -> i32 {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;
        use std::cmp::max;
        events.sort_by(|a,b| a[0].cmp(&b[0]));
        let mut ret = 0;
        let mut buffer = BinaryHeap::new();
        for i in 0..k {
            let mut next = BinaryHeap::new();
            let mut best = 0;
            for e in events.iter() {
                let start = e[0];
                let end = e[1];
                let value = e[2];
                while let Some(&(Reverse(e), v)) = buffer.peek() {
                    if e >= start {
                        break;
                    }
                    best = max(best, v);
                    buffer.pop();
                }
                let score = value + best;
                ret = max(ret, score);
                next.push((Reverse(end), score));
            }
            buffer = next;
        }
        return ret;
    }
}
