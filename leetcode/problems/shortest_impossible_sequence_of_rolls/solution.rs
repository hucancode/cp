impl Solution {
    pub fn shortest_sequence(rolls: Vec<i32>, k: i32) -> i32 {
        let mut vis = vec![false; 1 + k as usize];
        let mut ret = 1;
        let mut count = 0;
        for &x in rolls.iter() {
            let x = x as usize;
            if vis[x] {
                continue;
            }
            vis[x] = true;
            count += 1;
            if count != k {
                continue;
            }
            count = 0;
            ret += 1;
            vis.fill(false);
        }
        return ret;
    }
}