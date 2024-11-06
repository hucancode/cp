impl Solution {
    pub fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
        let MOD = 1000_000_007i64;
        let MAX_HAT = 40;
        let prefered_hats: Vec<i64> = hats.into_iter()
            .map(|a| a.iter().fold(0i64, |acc, x| acc | (1<<x)))
            .collect();
        let n = prefered_hats.len();
        let k = 1<<n;
        let mut f = vec![0i64;k];
        f[0] = 1;
        for hat in 1..=MAX_HAT {
            let mut g = f.clone();
            for mask in 0..k {
                for person in (0..n)
                    .filter(|&p| prefered_hats[p] & (1<<hat) != 0)
                    .filter(|&p| mask & (1<<p) == 0) {
                    //println!("let person {person} wear hat {hat}");
                    let next = mask | (1<<person);
                    g[next] += f[mask];
                    g[next] %= MOD;
                }
            }
            f = g;
            //println!("{f:?}");
        }
        f.last().map_or(0, |&x| x as i32)
    }
}
