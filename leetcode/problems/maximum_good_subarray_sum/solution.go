func maximumSubarraySum(nums []int, k int) int64 {
    prefix := make([]int64,len(nums)+1)
    for i,x := range nums {
        prefix[i+1] = prefix[i]+int64(x)
    }
    f := make(map[int]int64)
    var found = false
    var ret int64 = math.MinInt64
    for i,x := range nums {
        if v, ok := f[x+k]; ok {
            ret = max(ret, prefix[i+1]-v)
            found = true
        }
        if v, ok := f[x-k]; ok {
            ret = max(ret, prefix[i+1]-v)
            found = true
        }
        if v, ok := f[x]; ok {
            f[x] = min(v, prefix[i])
        } else {
            f[x] = prefix[i]
        }
    }
    if found {
        return ret
    }
    return 0
}