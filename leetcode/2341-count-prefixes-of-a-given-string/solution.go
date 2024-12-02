func countPrefixes(words []string, s string) int {
    ret := 0
    for _, w := range words {
        if len(s) >= len(w) && s[:len(w)] == w {
            ret++
        }
    }
    return ret
}
