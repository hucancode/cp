func timeRequiredToBuy(tickets []int, k int) int {
    ret := 0
    for i, x := range tickets {
        if i > k {
            ret += min(x, tickets[k]-1)
        } else {
            ret += min(x, tickets[k])
        }
    }
    return ret
}
