package bookstore

import (
	"fmt"
	"sort"
    "math"
)

func calculate(counts []int) int {
	const BASE_PRICE = 800.0
	//fmt.Printf("solving with counts = %v\n", counts)
	n := 0
	ret := float32(0.0)
	fmt.Printf("counts %v\n", counts)
	for i, x := range counts {
		m := len(counts) - i
		x -= n
		n += x
		fmt.Printf("there are %d books in each %d kinds\n", x, m)
		price := BASE_PRICE * float32(x) * float32(m)
		switch m {
		case 2:
			price *= 0.95
		case 3:
			price *= 0.9
		case 4:
			price *= 0.8
		case 5:
			price *= 0.75
		default:
		}
		ret += price
		fmt.Printf("price %f\n", ret)
	}
	return int(ret)
}
func min(x, y int) int {
  return int(math.Min(float64(x), float64(y)))
}
func Cost(books []int) int {
	counts := make([]int, 5)
	for _, book := range books {
		counts[book-1]++
	}
	sort.Ints(counts)
	delta := min(counts[0], counts[2]-counts[1])
	counts[0] -= delta
	counts[1] += delta
	return calculate(counts)
}