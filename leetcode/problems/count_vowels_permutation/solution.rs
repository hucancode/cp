impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let INF: i32 = 1000_000_007;
        let a = 0;
        let e = 1;
        let i = 2;
        let o = 3;
        let u = 4;
        let n = n as usize;
        let mut f = vec![1;5];
        for _ in 1..n {
            let mut g = vec![0;5];
            g[a] = ((f[e]+f[i])%INF+f[u])%INF;
            g[e] = (f[a]+f[i])%INF;
            g[i] = (f[e]+f[o])%INF;
            g[o] = f[i];
            g[u] = (f[i]+f[o])%INF;
            f = g;
        }
        return f.iter().fold(0, |acc,x| (acc+x)%INF);
    }
}