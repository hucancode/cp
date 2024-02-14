func minimumWhiteTiles(floor string, numCarpets int, carpetLen int) int {
    n := len(floor)
    f := make([]int, n+1)
    for i := range floor {
        f[i+1] = f[i] + int(floor[i] - '0')
    }
    for i := 0;i<numCarpets;i++ {
        next := make([]int, n+1)
        for j := 1;j<=n;j++ {
            cover := f[max(0, j-carpetLen)]
            skip := next[j-1] +  int(floor[j-1] - '0')
            if skip < cover {
                next[j] = skip
            } else {
                next[j] = cover
            }
        }
        f = next
    }
    return f[n]
}