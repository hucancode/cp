func gridIllumination(n int, lamps [][]int, queries [][]int) []int {
    lampByXY := make(map[[2]int]struct{})
    lampByRow := make(map[int]int)
    lampByCol := make(map[int]int)
    lampByDeltaXY := make(map[int]int)
    lampByDeltaMXY := make(map[int]int)

    for _, l := range lamps {
        lx, ly := l[0], l[1]
        pos := [2]int{lx, ly}
        if _, exists := lampByXY[pos]; !exists {
            lampByXY[pos] = struct{}{}
            lampByRow[lx]++
            lampByCol[ly]++
            lampByDeltaXY[lx-ly]++
            lampByDeltaMXY[-lx-ly]++
        }
    }

    ret := make([]int, len(queries))

    for qi, q := range queries {
        qx, qy := q[0], q[1]
        lit := lampByRow[qx] > 0 ||
            lampByCol[qy] > 0 ||
            lampByDeltaXY[qx-qy] > 0 ||
            lampByDeltaMXY[-qx-qy] > 0
        if lit {
            ret[qi] = 1
        } else {
            ret[qi] = 0
        }

        for x := qx-1; x <= qx+1; x++ {
            for y := qy-1; y <= qy+1; y++ {
                pos := [2]int{x, y}
                if _, exists := lampByXY[pos]; exists {
                    delete(lampByXY, pos)
                    lampByRow[x]--
                    lampByCol[y]--
                    lampByDeltaXY[x-y]--
                    lampByDeltaMXY[-x-y]--
                }
            }
        }
    }

    return ret
}
