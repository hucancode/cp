func evalRPN(tokens []string) int {
    var st []int
    for _, s := range tokens { 
        switch s {
        case "+":
            b := st[len(st)-1]
            a := st[len(st)-2]
            st[len(st)-2] = a+b
            st = st[:len(st)-1]
        case "-":
            b := st[len(st)-1]
            a := st[len(st)-2]
            st[len(st)-2] = a-b
            st = st[:len(st)-1]
        case "*":
            b := st[len(st)-1]
            a := st[len(st)-2]
            st[len(st)-2] = a*b
            st = st[:len(st)-1]
        case "/":
            b := st[len(st)-1]
            a := st[len(st)-2]
            st[len(st)-2] = a/b
            st = st[:len(st)-1]
        default:
            if i, err := strconv.Atoi(s); err == nil {
                st = append(st, i)
            }
        }
    }
    return st[len(st)-1]
}