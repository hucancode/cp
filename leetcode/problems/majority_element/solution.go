func majorityElement(nums []int) int {
    ret := nums[0]
    freq := 0
    for _, x := range nums {
        if freq == 0 {
            ret = x
        }
        if ret == x {
            freq++
        } else {
            freq--
        }
    }
    return ret
}