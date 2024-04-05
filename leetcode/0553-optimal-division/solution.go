import "strings"
func optimalDivision(nums []int) string {
    var strs []string
    for _, x := range nums {
        strs = append(strs, strconv.Itoa(x))
    }
    if len(strs) < 3 {
        return strings.Join(strs, "/")
    }
    return strs[0]+"/("+strings.Join(strs[1:], "/")+")"
}
