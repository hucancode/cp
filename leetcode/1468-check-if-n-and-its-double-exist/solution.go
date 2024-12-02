func checkIfExist(arr []int) bool {
    sort.Ints(arr)
    for i, x := range arr {
        x *= 2
        j := sort.Search(len(arr), func(i int) bool {
            return arr[i] >= x
        })
        if i != j && j < len(arr) && arr[j] == x {
            return true
	    }
    }
    return false
}
