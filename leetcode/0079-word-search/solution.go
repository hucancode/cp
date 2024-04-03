func exist(board [][]byte, word string) bool {
    n := len(board);
    m := len(board[0]);
    for i := 0; i < n; i++ {
        for j := 0; j < m; j++ {
            vis := make([][]bool, n)
            for i := range vis {
                vis[i] = make([]bool, m)
            }
            q := []int{ i, j, 0 }
            for len(q) >= 3 {
                nq := len(q)
                i,j,k := q[nq-3], q[nq-2], q[nq-1]
                q = q[:nq-3]
                if k < 0 {
                    vis[i][j] = false
                    continue
                }
                if k >= len(word) {
                    return true
                }
                if i < 0 || i >= n || j < 0 || j >= m {
                    continue
                }
                if vis[i][j] {
                    continue
                }
                if board[i][j] != word[k] {
                    continue
                }
                vis[i][j] = true
                q = append(q,i, j, -1)
                q = append(q,i+1, j, k+1)
                q = append(q,i-1, j, k+1)
                q = append(q,i, j+1, k+1)
                q = append(q,i, j-1, k+1)
            }
        }
    }
    return false
}
