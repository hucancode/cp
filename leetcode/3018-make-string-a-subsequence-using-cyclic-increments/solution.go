func canMakeSubsequence(str1 string, str2 string) bool {
    n := len(str1)
    m := len(str2)
    i := 0
    j := 0
    for i < n && j < m {
        if str1[i] == str2[j] || 
            str1[i] + 1 == str2[j] || 
            (str1[i] == 'z' && str2[j] == 'a') {
            j++
        }
        i++
    }
    return j == m
}
