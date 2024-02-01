func divideArray(nums []int, k int) [][]int {
    slices.Sort(nums)
    var ret [][]int
    for i := 0; i < len(nums); i += 3 {
        var chunk = nums[i:i+3]
        if slices.Max(chunk) - slices.Min(chunk) > k {
            return make([][]int, 0)
        }
        ret = append(ret, chunk)
    }
    return ret
}