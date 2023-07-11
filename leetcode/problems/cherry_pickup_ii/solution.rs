use std::cmp::max;
impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut f = vec![vec![i32::MIN;m];m];
        f[0][m-1] = grid[0][0] + grid[0][m-1];
        let moves = vec![
            (-1,-1),(-1,0),(-1,1),
            (0,-1),(0,0),(0,1),
            (1,-1),(1,0),(1,1),
        ];
        for i in 1..n {
            let g = f.clone();
            for a in 0..m {
                for b in a+1..m {
                    for (aj, bj) in moves.iter()
                        .map(|(ja, jb)| (a as i32 + ja, b as i32 + jb))
                        .filter(|&(aj, bj)| aj >= 0 && aj < m as i32 && bj >= 0 && bj < m as i32)
                        .map(|(aj, bj)| (aj as usize, bj as usize)) {
                        let x = g[aj][bj];
                        let y = grid[i][a] + grid[i][b];
                        f[a][b] = max(f[a][b], x+y);
                    }
                }
            }
        }
        let mut ret = i32::MIN;
        for row in f.into_iter() {
            for x in row.into_iter() {
                ret = max(ret, x);
            }
        }
        return ret;
    }
}