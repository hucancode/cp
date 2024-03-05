func minimumLength(s string) int {
    i := 0
    j := len(s) - 1
    for j > i {
        if s[i] != s[j] {
            break
        }
        i0 := i
        for i < j && s[i] == s[i0] {
            i++
        }
        for i <= j && s[j] == s[i0] {
            j--
        }
    }
    return j - i + 1
}
