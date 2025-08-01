package bookstore

import (
	"fmt"
	"sort"
)

func calculate(counts []int) int {
	fmt.Printf("solving with counts = %v\n", counts)
	n := 0
	ret := float32(0.0)
	fmt.Printf("counts %v\n", counts)
	if len(counts) == 5 {
		// there are a steep increase when we get 4 books so the best deal is to get 4 books as much as possible
		// try to redistribute books so group of 4 get maximum length
		delta := counts[2] - counts[1]
		if delta > 0 {
			if counts[0] > delta {
				counts[0] -= delta
				counts[1] += delta
			} else {
				counts[1] += counts[0]
				counts[0] = 0
			}
		}
	}
	for i, x := range counts {
		m := len(counts) - i
		x -= n
		n += x
		fmt.Printf("there are %d books in each %d kinds\n", x, m)
		switch m {
		case 2:
			ret += 0.95 * 2.0 * 800.0 * float32(x)
		case 3:
			ret += 0.9 * 3.0 * 800.0 * float32(x)
		case 4:
			ret += 0.8 * 4.0 * 800.0 * float32(x)
		case 5:
			ret += 0.75 * 5.0 * 800.0 * float32(x)
		default:
			ret += 800.0 * float32(x)
		}
		fmt.Printf("price %f\n", ret)
	}
	return int(ret)

}
func Cost(books []int) int {
	countByTitle := make(map[int]int)
	for _, book := range books {
		if _, ok := countByTitle[book]; !ok {
			countByTitle[book] = 0
		}
		countByTitle[book]++
	}
	counts := make([]int, 0)
	for _, count := range countByTitle {
		counts = append(counts, count)
	}
	sort.Ints(counts)
	return calculate(counts)
}
