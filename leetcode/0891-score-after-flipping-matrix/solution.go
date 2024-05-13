func matrixScore(grid [][]int) int {
    n := len(grid)
    m := len(grid[0])
    flipRow := make([]int, n)
    flipColumn := make([]int, m)
    for i:= 0;i<n;i++ {
        if grid[i][0] == 0 {
            flipRow[i] = 1
        }
    }
    for j := 0;j<m;j++ {
        oneCount := 0
        for i := 0;i<n;i++ {
            oneCount += (grid[i][j] + flipRow[i])%2
        }
        if oneCount*2 < n {
            flipColumn[j] = 1
        }
    }
    ret := 0
    for i := 0;i<n;i++ {
        k := 0
        for j := 0;j<m;j++ {
            x := (grid[i][j] + flipRow[i] + flipColumn[j])%2
            k = (k<<1) + x
        }
        ret += k
    }
    return ret
}
