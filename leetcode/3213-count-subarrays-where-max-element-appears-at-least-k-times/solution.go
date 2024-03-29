func countSubarrays(nums []int, k int) int64 {
    m := slices.Max(nums)
    var q []int
    var ret int64 = 0
    for i, x := range nums {
        if x == m {
            q = append(q, i)
        }
        for len(q) > k {
            q = q[1:]
        }
        if len(q) >= k {
            ret += int64(q[0]) + 1
        }
    }
    return ret
}
