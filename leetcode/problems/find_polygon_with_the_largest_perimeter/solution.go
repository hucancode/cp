func largestPerimeter(nums []int) int64 {
    slices.Sort(nums)
    var ret int64 = -1
    var sum int64 = 0
    for _,x := range nums {
        if sum > int64(x) {
            ret = max(ret, sum + int64(x))
        }
        sum += int64(x)
    }
    return ret
}