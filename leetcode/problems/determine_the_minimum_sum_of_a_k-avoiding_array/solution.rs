impl Solution {
    pub fn minimum_sum(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut vis = vec![false;101];
        let mut arr = Vec::new();
        let mut i = 1;
        while arr.len() < n as usize {
            if !vis[i] {
                arr.push(i);
                if i < k {
                    vis[k-i] = true;
                }
            }
            i+=1;
        }
        arr.into_iter().sum::<usize>() as i32
    }
}