package knapsack
import (
    "sort"
)
type Item struct {
	Weight, Value int
}

func max(a, b int) int {
    if a > b {
        return a
    }
    return b
}
// Knapsack takes in a maximum carrying capacity and a collection of items
// and returns the maximum value that can be carried by the knapsack
// given that the knapsack can only carry a maximum weight given by maximumWeight
func Knapsack(maximumWeight int, items []Item) int {
	f := make([][]int, maximumWeight+1)
    for i := 0; i<= maximumWeight; i++ {
        f[i] = make([]int, len(items)+1)
    }
    sort.Slice(items, func (i, j int) bool {
        return items[i].Weight < items[j].Weight
    })
    for i := 1; i<= maximumWeight; i++ {
        for j := 1; j <= len(items); j++ {
            w := items[j-1].Weight
            v := items[j-1].Value
            if w > i {
                f[i][j] = f[i][j-1]
            } else {
                f[i][j] = max(f[i-w][j-1]+v, max(f[i-1][j], f[i][j-1]))
            }
        }
    }
    return f[maximumWeight][len(items)]
}
