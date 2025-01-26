package bookstore

import (
	"fmt"
	"sort"
)

func calculate(counts []int) int {
	const BASE_PRICE = 800.0
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
