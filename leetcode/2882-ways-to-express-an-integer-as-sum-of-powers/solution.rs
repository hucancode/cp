impl Solution {
    pub fn number_of_ways(n: i32, x: i32) -> i32 {
        let x = x as u32;
        let INF = 1000_000_007;
        let mut f = vec![0;1 + n as usize];
        f[0] = 1;
        for k in (1..=n).map(|i| i.pow(x)).take_while(|&k| k <= n) {
            let mut g = f.clone();
            for i in 0..n {
                let j = k + i;
                if j <= n {
                    g[j as usize] += f[i as usize];
                    g[j as usize] %= INF;
                }
            }
            f = g;
        }
        return f[n as usize];
    }
}
