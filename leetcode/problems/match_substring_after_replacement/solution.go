func matchReplacement(s string, sub string, mappings [][]byte) bool {
    var f [256][256]bool
    for _,a := range mappings {
        k := a[0]
        v := a[1]
        f[k][v] = true
    }
    n := len(s)
    m := len(sub)
    for i := 0; i <= (n-m); i++ {
        j := 0
        for ; j < m; j++ {
            k := sub[j]
            v := s[i+j]
            if k != v && !f[k][v] {
                break
            }
        }
        if j == m {
            return true
        }
    }
    return false
}