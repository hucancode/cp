use std::collections::BinaryHeap;
use std::collections::VecDeque;
impl Solution {
    pub fn count_servers(n: i32, mut logs: Vec<Vec<i32>>, x: i32, queries: Vec<i32>) -> Vec<i32> {
        let m = queries.len();
        let mut task: Vec<usize> = (0..m).collect();
        let mut ans = vec![0;m];
        let mut request = vec![0;1 + n as usize];
        let mut q = VecDeque::new();
        let mut count = n;
        logs.sort_by(|a,b| b[1].cmp(&a[1]));
        task.sort_by(|&i,&j| queries[i].cmp(&queries[j]));
        for (i,t) in task.into_iter().map(|i| (i, queries[i])) {
            while let Some(log) = logs.last() {
                let server = log[0] as usize;
                let tx = log[1];
                if tx <= t {
                    logs.pop();
                } else {
                    break;
                }
                q.push_back((server, tx));
                request[server] += 1;
                if request[server] == 1 {
                    count -= 1;
                }
            }
            while let Some(log) = q.front() {
                let &(server, tx) = log;
                if tx < t - x {
                    q.pop_front();
                } else {
                    break;
                }
                request[server] -= 1;
                if request[server] == 0 {
                    count += 1;
                }
            }
            ans[i] = count;
        }
        return ans;
    }
}