impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let n = arr.len();
        let mut q = Vec::new();
        let mut vis = vec![false;n];
        q.push(start);
        while let Some(u) = q.pop() {
            if u >= n as i32 || u < 0 {
                continue;
            }
            if vis[u as usize] {
                continue;
            }
            vis[u as usize] = true;
            let d = arr[u as usize];
            if d == 0 {
                return true;
            }
            let v1 = u + d;
            let v2 = u - d;
            q.push(v1);
            q.push(v2);
        }
        return false;
    }
}