package isogram

func IsIsogram(word string) bool {
	seen := [26]bool{}
    for _, c := range([]byte(word)) {
        if c == ' ' || c == '-' {
            continue
        }
        i := byte(0)
        if c <= byte('z') && c >= byte('a') {
            i = c - byte('a')
        }
        if c <= byte('Z') && c >= byte('A') {
            i = c - byte('A')
        }
        if seen[i] {
            return false
        }
        seen[i] = true
    }
    return true
}
