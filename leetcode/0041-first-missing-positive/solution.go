func firstMissingPositive(nums []int) int {
    n := len(nums)
    for i,x  := range nums {
        nums[i] = 0
        for x >= 1 && x <= n && nums[x-1] != x {
            // pluck
            next := nums[x-1]
            // put 
            nums[x-1] = x
            x = next
        }
    }
    ret := 1
    for ret <= n && nums[ret-1] == ret {
        ret++
    }
    return ret
}
