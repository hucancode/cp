func longestNiceSubarray(nums []int) int {
    q := []int{} 
    sum := 0
    ret := 0
    for _, x := range nums {
        for len(q) > 0 && sum & x != 0 {
            sum = sum &^ q[0]
            q = q[1:]
        }
        sum = sum | x
        q = append(q, x)
        ret = max(ret, len(q))
    }
    return ret
}