func minimumMountainRemovals(nums []int) int {
    n := len(nums)
    f := make([]int, n)
    g := make([]int, n)
    for i := 0;i<n;i++ {
        for j := i-1;j>=0;j-- {
            if nums[i] > nums[j] {
                f[i] = max(f[i], f[j]+1)
            }
        }
    }
    for i := n-1;i>=0;i-- {
        for j := i+1;j<n;j++ {
            if nums[i] > nums[j] {
                g[i] = max(g[i], g[j]+1)
            }
        }
    }
    //fmt.Println(f,g);
    ret := n-1
    for i:= 1;i<n-1;i++ {
        if f[i] == 0 || g[i] == 0 {
            continue
        }
        ret = min(ret, n-f[i]-g[i]-1)
    }
    return ret
}
