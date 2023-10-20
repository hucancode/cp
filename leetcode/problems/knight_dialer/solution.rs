impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        let INF: i32 = 1000_000_007;
        let adj = vec![
            vec![4,6], //0
            vec![8,6], //1
            vec![7,9], //2
            vec![4,8], //3
            vec![0,3,9], //4
            vec![], //5
            vec![0,1,7], //6
            vec![2,6], //7
            vec![1,3], //8
            vec![2,4], //9
        ];
        let mut f = vec![1;10];
        for i in 1..n {
            let mut g = vec![0;10];
            for u in 0..10 {
                for &v in adj[u].iter() {
                    g[u] += f[v];
                    g[u] %= INF;
                }
            }
            f = g;
        }
        return f.iter().fold(0, |acc,x| (acc+x)%INF);
    }
}