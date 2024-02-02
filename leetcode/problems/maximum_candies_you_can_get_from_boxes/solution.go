func maxCandies(status []int, candies []int, keys [][]int, containedBoxes [][]int, initialBoxes []int) int {
    var q []int
    var n = len(status)
    var haveBox = make([]bool, n)
    var haveKey = make([]bool, n)
    var vis = make([]bool, n)
    var ret = 0
    for _,i := range initialBoxes {
        haveBox[i] = true
        if status[i] == 1 {
            q = append(q, i)
        }
    }
    for len(q) > 0 {
        var i = q[0]
        q = q[1:]
        if vis[i] {
            continue
        }
        ret += candies[i]
        vis[i] = true
        for _,j := range containedBoxes[i] {
            haveBox[j] = true;
            if haveKey[j] || (status[j] == 1) {
                q = append(q, j)
            }
        }
        for _,j := range keys[i] {
            haveKey[j] = true
            if haveBox[j] {
                q = append(q, j)
            }
        }
    }
    return ret
}