impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        use std::collections::VecDeque;
        use std::collections::HashSet;
        let target = target.parse::<usize>().unwrap();
        let deadends: HashSet<usize> = deadends.into_iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let mut vis = vec![false;10000];
        let mut q = VecDeque::new();
        q.push_back((0,0));
        while let Some((x,cost)) = q.pop_front() {
            if vis[x] {
                continue;
            }
            vis[x] = true;
            if deadends.contains(&x) {
                continue;
            }
            if x == target {
                return cost;
            }
            let mut k = 1;
            while k <= 1000 {
                let d = x%(k*10)/k;
                if d < 9 {
                    q.push_back((x+k, cost+1));
                } else {
                    q.push_back((x-k*9, cost+1));
                }
                if d > 0 {
                    q.push_back((x-k, cost+1));
                } else {
                    q.push_back((x+k*9, cost+1));
                }
                k *= 10;
            }
        }
        return -1;
    }
}
