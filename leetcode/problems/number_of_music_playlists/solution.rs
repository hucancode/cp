impl Solution {
    pub fn num_music_playlists(n: i32, goal: i32, k: i32) -> i32 {
        let goal = goal as usize;
        let n = n as usize;
        let k = k as usize;
        let INF = 1000_000_007;
        let mut f = vec![vec![0;1+n]; 1+goal];
        f[0][0] = 1;
        for i in 1..=goal {
            for j in 1..=n {
                let add_new = n-j+1;
                let replay = if j > k { j - k } else { 0 };
                f[i][j] += f[i-1][j-1] * add_new as i64;
                f[i][j] += f[i-1][j] * replay as i64;
                f[i][j] %= INF;
            }
        }
        return f[goal][n] as i32;
    }
}