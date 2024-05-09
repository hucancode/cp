func maximumHappinessSum(happiness []int, k int) int64 {
	sort.Sort(sort.Reverse(sort.IntSlice(happiness)))
    ret := int64(0)
    for i:= 0;i<k;i++ {
        ret += int64(max(0, happiness[i] - i))
    }
    return ret
}
