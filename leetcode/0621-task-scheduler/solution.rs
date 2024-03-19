use std::collections::BinaryHeap;
impl Solution {
    pub fn least_interval(mut tasks: Vec<char>, n: i32) -> i32 {
        let mut q = BinaryHeap::new();
        let mut buffer = BinaryHeap::new();
        tasks.sort();
        let mut c = 0;
        let mut last = 'A';
        for t in tasks {
            if t == last {
                c += 1;
            } else if c != 0 {
                q.push(c);
                c = 1;
            }
            last = t;
        }
        q.push(c);
        let mut ret = 0;
        while !q.is_empty() {
            let mut k = 0;
            while let Some(t) = q.pop() {
                if t > 1 {
                    buffer.push(t-1);
                }
                k += 1;
                if k == n+1 {
                    break;
                }
            }
            ret += if buffer.is_empty() {k} else {n+1};
            while let Some(t) = buffer.pop() {
                q.push(t);
            }
        }
        ret
    }
}
