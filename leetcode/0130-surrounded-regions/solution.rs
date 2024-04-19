impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let n = board.len();
        let m = board[0].len();
        let mut vis = vec![vec![false;m];n];
        for i in 0..n {
            for j in 0..m {
                if board[i][j] == 'X' {
                    continue;
                }
                let mut surrounded = true;
                let mut q = vec![(i,j)];
                let mut buffer = Vec::new();
                while let Some((i,j)) = q.pop() {
                    if vis[i][j] || board[i][j] == 'X' {
                        continue;
                    }
                    vis[i][j] = true;
                    if i == 0 || j == 0 || i == n-1 || j == m-1 {
                        surrounded = false;
                    }
                    buffer.push((i,j));
                    if i > 0 {
                        q.push((i-1, j));
                    }
                    if i < n-1 {
                        q.push((i+1, j));
                    }
                    if j > 0 {
                        q.push((i, j-1));
                    }
                    if j < m-1 {
                        q.push((i, j+1));
                    }
                }
                if surrounded {
                    while let Some((i,j)) = buffer.pop() {
                        board[i][j] = 'X';
                    }
                }
            }
        }
    }
}
