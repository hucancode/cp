func groupAnagrams(strs []string) [][]string {
    f := make(map[[26]int][]string)
    for _,s := range strs {
        var k [26]int
        for _, c := range s {
            k[c-'a']++
        }
        f[k] = append(f[k], s)
    }
    var ret [][]string
    for _,v := range f {
        ret = append(ret, v)
    }
    return ret
}