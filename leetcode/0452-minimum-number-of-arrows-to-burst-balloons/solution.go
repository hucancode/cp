func findMinArrowShots(points [][]int) int {
    sort.Slice(points, func(i, j int) bool {
		return points[i][0] < points[j][0]
	})
    //fmt.Println(points)
    x := math.MaxInt
    ret := 0
    for _,r := range(points) {
        a := r[0]
        b := r[1]
        if a > x {
            //fmt.Println("burst at", x)
            ret++
            x = math.MaxInt
        }
        x = min(x, b)
    }
    return ret + 1
}
