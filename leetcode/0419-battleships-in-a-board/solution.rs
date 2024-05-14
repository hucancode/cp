impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let mut ret = 0;
        let n = board.len();
        let m = board[0].len();
        for i in 0..n {
            for j in 0..m {
                if board[i][j] != 'X' {
                    continue;
                }
                if i > 0 && board[i-1][j] == 'X' {
                    continue;
                }
                if j > 0 && board[i][j-1] == 'X' {
                    continue;
                }
                ret += 1;
            }
        }
        ret
    }
}
