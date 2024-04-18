func islandPerimeter(grid [][]int) int {
    n := len(grid)
    m := len(grid[0])
    x := 0
    y := 0
    vis := make([][]bool, n)
    for i := range grid {
        vis[i] = make([]bool, m)
        for j := range grid[i] {
            vis[i][j] = false
        }
    }
    for i := range grid {
        for j := range grid[i] {
            if grid[i][j] == 1 {
                x = i
                y = j
                break
            }
        }
        if grid[x][y] == 1 {
            break;
        }
    }
    ret := 0
    q := []int {x,y}
    for len(q) != 0 {
        x := q[0]
        y := q[1]
        q = q[2:]
        if x < 0 || x >= n || y < 0 || y >= m || grid[x][y] == 0 {
            ret++
            continue
        }
        if vis[x][y] {
            continue
        }
        vis[x][y] = true
        q = append(q, x+1, y, x-1, y, x, y+1, x, y-1)
    }
    return ret
}
