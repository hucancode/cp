func subsetXORSum(nums []int) int {
    ret := 0
    for _, x := range nums {
        ret |= x
    }
    return ret << (len(nums) - 1)
}
