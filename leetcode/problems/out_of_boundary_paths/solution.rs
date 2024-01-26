use std::collections::HashSet;
impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let INF: i32 = 1000_000_007;
        let moves = vec![(-1,0), (1,0), (0,-1), (0,1)];
        let m = m as usize;
        let n = n as usize;
        let start_row = start_row as usize;
        let start_column = start_column as usize;
        let mut f = vec![vec![0;n];m];
        f[start_row][start_column] = 1;
        let mut q = HashSet::new();
        q.insert((start_row, start_column));
        let mut ret = 0;
        for i in 0..max_move {
            let mut f_next = vec![vec![0;n];m];
            let mut q_next = HashSet::new();
            for (x, y) in q.into_iter() {
                for (dx, dy) in moves.iter() {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx < 0 || ny < 0 || nx >= m as i32  || ny >= n as i32  {
                        ret += f[x][y];
                        ret %= INF;
                    } else {
                        f_next[nx as usize][ny as usize] += f[x][y];
                        f_next[nx as usize][ny as usize] %= INF;
                        q_next.insert((nx as usize,ny as usize));
                    }
                }
            }
            f = f_next;
            q = q_next;
        }
        return ret;
    }
}