func rangeBitwiseAnd(left int, right int) int {
    ret := 0
    for i := 0;i<31;i++ {
        k := 1<<i
        x := left/k
        y := right/k
        if x == y && x%2 == 1 {
            ret |= k
        }
    }
    return ret
}