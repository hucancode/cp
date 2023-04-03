impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let n = stones.len();
        let mut f = vec![vec![];n];
        f[0].push(0);
        for i in 0..n {
            for k in 0..f[i].len() {
                for d in [1, 0, -1].iter() {
                    let jump = f[i][k] + d;
                    if jump <= 0 {
                        continue;
                    }
                    let target = stones[i]+jump;
                    match stones.binary_search(&target) {
                        Ok(j) => {
                            match f[j].binary_search(&jump) {
                                Err(i) => f[j].insert(i, jump),
                                _ => {},
                            };
                        },
                        _ => {},
                    }
                }
            }
        }
        return !f.last().unwrap().is_empty();
    }
}