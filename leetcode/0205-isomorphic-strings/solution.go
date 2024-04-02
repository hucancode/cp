func isIsomorphic(s string, t string) bool {
    s2t := make(map[byte]byte)
    t2s := make(map[byte]byte)
    for i := range s {
        si := s[i]
        ti := t[i]
        sx,s_mapped := s2t[si]
        _,t_mapped := t2s[ti]
        if s_mapped != t_mapped {
            return false
        }
        if s_mapped && sx != ti {
            return false
        }
        s2t[si] = ti
        t2s[ti] = si
    }
    return true
}
