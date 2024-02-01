func maxLength(arr []string) int {
    var masks []int
    for _, s := range arr {
        var mask = 0
        for _, i := range s {
            i -= 'a'
            if mask & (1<<i) != 0 {
                mask = 0
                break
            }
            mask |= 1<<i
        }
        if mask != 0 {
            masks = append(masks, mask)
        }
    }
    var n = 1<<len(masks)
    var f = make([]int, n)
    for picked := 0; picked < n; picked++ {
        for i := 0; i< len(masks); i++ {
            if masks[i] & f[picked] != 0 {
                continue
            }
            var next = picked | (1<<i)
            f[next] = f[picked] | masks[i]
        }
    }
    var ret = 0
    for i := 0; i < n; i++ {
        var k = bits.OnesCount(uint(f[i]))
        if k > ret {
            ret = k
        }
    }
    return ret
}