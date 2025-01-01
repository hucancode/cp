func maxScore(s string) int {
    n := len(s)
	f := make([]int, n+1)
	for i, c := range s {
        f[i+1] = f[i]
		if c == '0' {
			f[i+1]++
		}
	}
	ret := 0
	for i := 1; i < n; i++ {
		score := n - i - (f[n] - f[i]) + f[i]
		if score > ret {
			ret = score
		}
	}
	return ret
}
