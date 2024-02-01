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
    for i, picked := range f {
        for j, picking := range masks {
            if picking & picked != 0 {
                continue
            }
            var next = i | (1<<j)
            f[next] = picking | picked
        }
    }
    var ret = 0
    for _, mask := range f {
        var k = bits.OnesCount(uint(mask))
        if k > ret {
            ret = k
        }
    }
    return ret
}