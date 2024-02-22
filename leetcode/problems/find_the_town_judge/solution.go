func findJudge(n int, trust [][]int) int {
    trusted := make([]int, n+1)
    trusting := make([]int, n+1)
    for _, e := range trust {
        u := e[0]
        v := e[1]
        if u == v {
            continue
        }
        trusted[v]++
        trusting[u]++
    }
    ret := -1
    for i := 1;i<=n;i++ {
        if trusted[i] == n-1 && trusting[i] == 0 {
            if ret != -1 {
                ret = -1
                break
            }
            ret = i
        } 
    }
    return ret
}