type Class struct {
	Bonus float64
	Passed int
	Total  int
}

type MaxHeap []Class

func (h MaxHeap) Len() int           { return len(h) }
func (h MaxHeap) Less(i, j int) bool { return h[i].Bonus > h[j].Bonus }
func (h MaxHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *MaxHeap) Push(x interface{}) { *h = append(*h, x.(Class)) }
func (h *MaxHeap) Pop() interface{} {
	old := *h
	n := len(old)
	item := old[n-1]
	*h = old[:n-1]
	return item
}

func calculateBonus(passed, total int) float64 {
	p := float64(passed)
	n := float64(total)
	return (n - p) / n / (n + 1.0)
}

func maxAverageRatio(classes [][]int, extraStudents int) float64 {
	maxHeap := &MaxHeap{}
	heap.Init(maxHeap)

	// Populate the heap
	for _, class := range classes {
		passed, total := class[0], class[1]
		heap.Push(maxHeap, Class{
			Bonus:  calculateBonus(passed, total),
			Passed: passed,
			Total:  total,
		})
	}

	// Distribute extra students
	for extraStudents > 0 {
		class := heap.Pop(maxHeap).(Class)
		class.Passed++
		class.Total++
		class.Bonus = calculateBonus(class.Passed, class.Total)
		heap.Push(maxHeap, class)
		extraStudents--
	}

	// Calculate final average ratio
	totalRatio := 0.0
	for maxHeap.Len() > 0 {
		class := heap.Pop(maxHeap).(Class)
		totalRatio += float64(class.Passed) / float64(class.Total)
	}

	return totalRatio / float64(len(classes))
}
