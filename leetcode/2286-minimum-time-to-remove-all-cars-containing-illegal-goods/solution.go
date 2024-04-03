func minimumTime(s string) int {
    n := len(s)
    pick_mid := 0
    clear_left := 0
    ret := n
    for i,c := range s {
        clear_right := n-i-1
        if c == '1' {
            pick_mid = min(clear_left, pick_mid) + 2
            clear_left = i+1
        }
        ret = min(ret, min(pick_mid, clear_left) + clear_right)
    }
    return ret
}
