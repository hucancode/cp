func maximalRectangle(matrix [][]byte) int {
    n := len(matrix)
    m := len(matrix[0])
    rowStreak := make([][]int, n)
    for i:=0;i<n;i++ {
        rowStreak[i] = make([]int, m)
    }
    for i, row := range matrix {
        for j, x := range row {
            if(x == '0') {
                continue
            }
            if(j == 0) {
                rowStreak[i][j] = 1
            } else {
                rowStreak[i][j] = rowStreak[i][j-1] + 1
            }
        }
    }
    ret := 0
    for i, row := range rowStreak {
        for j, x := range row {
            h := 0
            w := x
            for i-h >= 0 && w*(i+1) > ret {
                w = min(w, rowStreak[i-h][j])
                ret = max(ret, w*(h+1))
                h++
            }
        }
    }
    return ret
}
