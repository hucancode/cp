use std::collections::VecDeque;
impl Solution {
    pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len() as i32;
        let mut q = VecDeque::new();
        q.push_back((0,0,1));
        while let Some((x, y, d)) = q.pop_front() {
            if x < 0 || x >= n || y < 0 || y >= n {
                continue;
            }
            let v = &mut grid[x as usize][y as usize];
            if *v == 1 {
                continue;
            }
            *v = 1;
            if x == n-1 && y == n-1 {
                return d;
            }
            for dx in -1..=1 {
                for dy in -1..=1 {
                    q.push_back((x+dx, y+dy, d+1));
                }
            }
        }
        return -1;
    }
}