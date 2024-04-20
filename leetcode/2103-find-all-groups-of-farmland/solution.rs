impl Solution {
    pub fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = land.len();
        let m = land[0].len();
        let mut vis = vec![vec![false;m];n];
        let mut ret = Vec::new();
        for i in 0..n {
            for j in 0..m {
                if land[i][j] != 1 || vis[i][j] {
                    continue;
                }
                let a = i;
                let b = j;
                let mut c = i;
                let mut d = j;
                while c < n && land[c][b] == 1 {
                    c += 1;
                }
                while d < m && land[a][d] == 1 {
                    d += 1;
                }
                ret.push(vec![a as i32,b as i32,c as i32 - 1,d as i32 - 1]);
                for i in a..c {
                    for j in b..d {
                        vis[i][j] = true;
                    }
                }
            }
        }
        ret
    }
}
