impl Solution {
    pub fn max_two_events(mut events: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;
        use std::cmp::max;
        events.sort_by(|a,b| a[0].cmp(&b[0]));
        let mut ret = 0;
        let mut buffer = BinaryHeap::new();
        let mut best = 0;
        for e in events {
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
            ret = max(ret, value + best);
            buffer.push((Reverse(end), value));
        }
        return ret;
    }
}
