impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word = word.as_bytes();
        let n = board.len();
        let m = board[0].len();
        for i in 0..n {
            for j in 0..m {
                let mut vis = vec![vec![false;m];n];
                let mut q = Vec::new();
                q.push((i,j,0));
                while let Some((i,j,k)) = q.pop() {
                    if k < 0 {
                        vis[i][j] = false;
                        continue;
                    }
                    if vis[i][j] {
                        continue;
                    }
                    if board[i][j] as u8 != word[k as usize] {
                        continue;
                    }
                    if k as usize >= word.len() - 1 {
                        return true;
                    }
                    vis[i][j] = true;
                    q.push((i, j, -1));
                    if i < n-1 { q.push((i+1, j, k+1)) };
                    if i > 0 { q.push((i-1, j, k+1)) };
                    if j < m-1 { q.push((i, j+1, k+1)) };
                    if j > 0 { q.push((i, j-1, k+1)) };
                }
            }
        }
        return false;
    }
}
