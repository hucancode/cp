func getMaxRepetitions(s1 string, n1 int, s2 string, n2 int) int {
    var n = len(s1)
    var m = len(s2)
    var f = make([]int, m)
    for j := 0; j < m; j++ {
        var k = 0
        for i := 0; i < n; i++ {
            if s1[i] == s2[(j+k)%m] {
                k++
            }
        }
        f[j] = k
	}
    var j = 0
    for i := 0; i < n1; i++ {
        j += f[j%m]
    }
    return j/m/n2
}