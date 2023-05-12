impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let n = stones.len();
        let mut f = vec![vec![];n];
        f[0].push(0);
        for i in 0..n {
            let m = f[i].len();
            for k in 0..m {
                for d in [1, 0, -1].iter() {
                    let jump = f[i][k] + d;
                    if jump <= 0 {
                        continue;
                    }
                    let target = stones[i]+jump;
                    if let Ok(j) = stones.binary_search(&target) {
                        if let Err(i) = f[j].binary_search(&jump) {
                            f[j].insert(i, jump);
                        }
                    }
                }
            }
        }
        return !f.last()
            .unwrap_or(&Vec::new())
            .is_empty();
    }
}