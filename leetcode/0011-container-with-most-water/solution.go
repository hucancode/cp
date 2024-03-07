func maxArea(height []int) int {
    l := 0
    r := len(height)-1
    ret := 0
    for l < r {
        ret = max(ret, (r-l)*min(height[l], height[r]))
        if height[l] > height[r] {
            r--
        } else {
            l++
        }
    }
    return ret
}
