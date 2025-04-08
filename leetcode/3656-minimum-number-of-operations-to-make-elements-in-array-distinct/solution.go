func minimumOperations(nums []int) int {
    var i int
    vis := make([]bool, 101)
    for i = len(nums)-1;i >= 0 && !vis[nums[i]];i-- {
        vis[nums[i]] = true
    }
    return (i+3)/3
}
