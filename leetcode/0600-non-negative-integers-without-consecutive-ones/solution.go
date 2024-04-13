func findIntegers(n int) int {
    f := [31]int{1,2} // f(i) = how many way to pick valid number not larger than 2^i
    for i:=2;i<31;i++ {
        f[i] = f[i-1] + f[i-2]
    }
    ret := 1
    //fmt.Println(f);
    for i:=30;i>=0;i-- {
        pattern := (n>>i) & 0b11
        if pattern == 0b11 {
            // can't pick 1 here, go to the next candidate not larger than n
            // 11xxxxxxxxx -> 00111111111
            n = 1<<i - 1
        } else if pattern == 0b01 {
            // can pick 1 here
            ret += f[i]
        }
    }
    return ret
}
