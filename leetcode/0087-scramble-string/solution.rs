impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let n = s1.len();
        let mut f = vec![vec![vec![false;n+1];n];n];
        // f[i][j][len] = can s1[i,i+len] scramble to s2[j, j+len]?
        for i in 0..n {
            for j in 0..n {
                if s1[i] == s2[j] {
                    f[i][j][1] = true;
                }
            }
        }
        for len in 2..=n {
            for i in 0..=n-len {
                for j in 0..=n-len {
                    for x in 1..len {
                        // no swap, 0..x vs 0..x, x..len vs x..len
                        let y = len-x;
                        f[i][j][len] |= f[i][j][x] && f[i+x][j+x][y];
                        // swap 0..y vs len-y..len, x..len vs 0..len-x
                        f[i][j][len] |= f[i][j+y][x] && f[i+x][j][y];
                        if f[i][j][len] {
                            break;
                        }
                    }
                }
            }
        }
        return f[0][0][n];
    }
}
