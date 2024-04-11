func removeKdigits(num string, k int) string {
    var ret []byte
    for i := range num {
        //fmt.Println("got ",x)
        for k > 0 && len(ret) > 0 && num[i] < ret[len(ret)-1] {
            ret = ret[:len(ret)-1]
            k--
        }
        is_leading_zero := num[i] == '0' && len(ret) == 0
        if !is_leading_zero {
            ret = append(ret, num[i])
        }
    }
    for k > 0 && len(ret) > 0 {
        ret = ret[:len(ret)-1]
        k--
    }
    if len(ret) == 0 {
        return "0"
    }
    return string(ret)
}
