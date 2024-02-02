func sequentialDigits(low int, high int) []int {
    var ret []int
    for len := 1; len <= 9; len++ {
        for i := 1; i <= 10-len; i++ {
            var k = 0
            for j := 0; j < len; j++ {
                k = k*10 + i+j
            }
            if k >= low && k <= high {
                ret = append(ret, k)
            }
        }
    }
    return ret
}