func maxSubarrayLength(nums []int, k int) int {
    f := make(map[int][]int)
    j := 0
    ret := 0
    for i,x := range nums {
        if len(f[x]) >= k {
            j = max(j, f[x][0]+1)
            f[x] = f[x][1:]
        }
        f[x] = append(f[x], i)
        ret = max(ret, i-j+1)
    }
    return ret
}
