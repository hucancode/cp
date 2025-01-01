impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        use std::collections::VecDeque;
        use std::cmp::min;
        let n = days.len();
        let mut f = vec![1000_000_000;n+1];
        let mut q7 = VecDeque::new();
        let mut q30 = VecDeque::new();
        f[0] = 0;
        for i in 1..=n {
            let d = days[i-1];
            while let Some(&prev) = q7.front() {
                if d - prev < 7 {
                    break;
                }
                q7.pop_front();
            }
            q7.push_back(d);
            while let Some(&prev) = q30.front() {
                if d - prev < 30 {
                    break;
                }
                q30.pop_front();
            }
            q30.push_back(d);
            f[i] = min(f[i], f[i-1] + costs[0]);
            f[i] = min(f[i], f[i-q7.len()] + costs[1]);
            f[i] = min(f[i], f[i-q30.len()] + costs[2]);
        }
        f[n]
    }
}
