func missingNumber(nums []int) int {
    sum := 0
    targetSum := len(nums)*(len(nums)+1)/2
    for _, x := range nums {
        sum += x
    }
    return targetSum - sum
}