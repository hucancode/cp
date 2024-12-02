func prefixCount(words []string, pref string) int {
    ret := 0
    for _, w := range words {
        if len(w) >= len(pref) && w[:len(pref)] == pref {
            ret += 1
        }
    }
    return ret
}
