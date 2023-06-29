use std::collections::VecDeque;
use std::cmp::max;
impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        let grid:Vec<Vec<char>> = grid.into_iter()
            .map(|s| { s.chars().collect() })
            .collect();
        const INF: i32 = 1000_000_000;
        const ALPHABET_OFFSET: usize = 'a' as usize;
        let n = grid.len();
        let m = grid[0].len();
        let mut k = 0;
        let mut q = VecDeque::new();
        for i in 0..n {
            for j in 0..m {
                let c = grid[i][j];
                if c == '@' {
                    let mask = 0;
                    let d = 1;
                    q.push_back((d, i+1, j, mask));
                    q.push_back((d, i-1, j, mask));
                    q.push_back((d, i, j+1, mask));
                    q.push_back((d, i, j-1, mask));
                }
                if c.is_lowercase() {
                    k = max(k, c as usize - ALPHABET_OFFSET + 1);
                }
            }
        }
        let k = 1<<k;
        let all_key_mask = k-1;
        let mut vis = vec![vec![vec![false;k];m];n];
        while let Some((d, i, j, mask)) = q.pop_front() {
            if i < 0 || i >= n || j < 0 || j >= m || vis[i][j][mask] {
                continue;
            }
            vis[i][j][mask] = true;
            let mut c = grid[i][j];
            let is_wall = c == '#';
            if is_wall {
                continue;
            }
            let is_lock = c.is_uppercase();
            let is_key = c.is_lowercase();
            let key_id = c.to_ascii_lowercase() as usize - ALPHABET_OFFSET;
            if is_lock {
                let no_key = mask & (1<<key_id) == 0;
                if no_key {
                    continue;
                }
            }
            //println!("visit {i}-{j} with hand = {mask:#6b}, d = {d}");
            let mut mask = mask;
            if is_key {
                mask |= 1<<key_id;
                //println!("collect key {c}({key_id}) at {i}-{j}, next hand = {mask:#6b}");
            }
            if mask == all_key_mask {
                return d;
            }
            let d = d+1;
            q.push_back((d, i+1, j, mask));
            q.push_back((d, i-1, j, mask));
            q.push_back((d, i, j+1, mask));
            q.push_back((d, i, j-1, mask));
        }
        return -1;       
    }
}