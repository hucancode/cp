func numIslands(grid [][]byte) int {
    n := len(grid)
    m := len(grid[0])
    vis := make([][]bool, n)
    for i := range vis {
        vis[i] = make([]bool, m)
    }
    ret := 0
    for i := range grid {
        for j := range grid[i] {
            if vis[i][j] || grid[i][j] == '0' {
                continue
            }
            ret++
            q := []int {i,j}
            for len(q) >= 2 {
                x := q[0]
                y := q[1]
                q = q[2:]
                if x < 0 || x >= n || y < 0 || y >= m || vis[x][y] || grid[x][y] == '0'{
                    continue
                }
                vis[x][y] = true
                q = append(q, x-1, y, x+1, y, x, y-1, x, y+1)
            }
        }
    }
    return ret
}
