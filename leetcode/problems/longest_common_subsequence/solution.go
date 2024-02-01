func longestCommonSubsequence(text1 string, text2 string) int {
    var n = len(text1)
    var m = len(text2)
    var f = make([][]int, n+1)
    for i := range f {
        f[i] = make([]int, m+1)
    }
    for i := 1;i<=n;i++ {
        for j := 1;j<=m;j++ {
            if text1[i-1] == text2[j-1] {
                f[i][j] = f[i-1][j-1] + 1
            } else {
                if f[i-1][j] > f[i][j] {
                    f[i][j] = f[i-1][j]
                }
                if f[i][j-1] > f[i][j] {
                    f[i][j] = f[i][j-1]
                }
            }
        }
    }
    return f[n][m]
}