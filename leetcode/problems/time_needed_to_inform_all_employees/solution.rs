use std::cmp::max;
impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut children = vec![Vec::new();n as usize];
        for (i, p) in manager.into_iter().enumerate() {
            if p == -1 {
                continue;
            }
            children[p as usize].push(i);
        }
        let mut ret = 0;
        let mut q = Vec::new();
        q.push((head_id as usize, 0));
        while let Some((u, du)) = q.pop() {
            ret = max(ret, du);
            let duv = inform_time[u];
            for &v in children[u].iter() {
                q.push((v, du+duv));
            }
        }
        ret
    }
}