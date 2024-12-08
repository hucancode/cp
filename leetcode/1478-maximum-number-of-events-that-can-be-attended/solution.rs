impl Solution {
    pub fn max_events(mut events: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::{VecDeque, BinaryHeap};
        events.sort_by(|a,b| a[0].cmp(&b[0]));
        let mut events: VecDeque<(usize, usize)> = events.into_iter()
            .map(|e| (e[0] as usize, e[1] as usize))
            .collect();
        let mut ret = 0;
        let mut buffer = BinaryHeap::new();
        let mut t = 0;
        for t in 1..=100_000 {
            while let Some(&(start, end)) = events.front() {
                if start > t {
                    break;
                }
                buffer.push(Reverse(end));
                events.pop_front();
            }
            while let Some(Reverse(e)) = buffer.pop() {
                if e >= t {
                    ret += 1;
                    break;
                }
            }
        }
        return ret;
    }
}
