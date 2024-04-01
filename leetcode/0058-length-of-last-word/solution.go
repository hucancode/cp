func lengthOfLastWord(s string) int {
    a := strings.Fields(s)
    if len(a) != 0 {
        return len(a[len(a)-1])
    }
    return 0
}
