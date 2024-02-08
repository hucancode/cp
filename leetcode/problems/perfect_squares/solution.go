func numSquares(n int) int {
    i := 1;
    var pool []int
    for i*i <= n {
        pool = append(pool, i*i)
        i++
    }
    f := make([]int, n+1)
    f[0] = n
    for i := 1; i < len(f); i *= 2 {
        copy(f[i:], f[:i])
    }
    f[0] = 0
    for i := range f {
        for _,x := range pool {
            if i + x > n {
                break
            }
            if f[i] + 1 < f[i+x] {
                f[i+x] = f[i] + 1
            }
        }
    }
    return f[n]
}