func customSortString(order string, s string) string {
    var count [26]int
    for _,c := range s {
        count[c-'a']++
    }
    ret := ""
    for _,c := range order {
        for i := 0;i<count[c-'a'];i++ {
            ret += string(c)
        }
        count[c-'a'] = 0
    }
    for i := 0;i<26;i++ {
        for j := 0;j<count[i];j++ {
            ret += string(i+'a')
        }
    }
    return ret
}
