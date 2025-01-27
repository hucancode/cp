module main

fn calculate(counts []int) int {
    mut total := 0.0
    mut used := 0
    base_price := 800
    n := counts.len
    for i := 0; i < n; i++ {
        m := n-i
        x := counts[i] - used
        used = counts[i]
        price := base_price * m * x
        discount := match m {
          2 { 0.95 }
          3 { 0.90 }
          4 { 0.80 }
          5 { 0.75 }
          else { 1.00 }
        }
        total += price * discount
    }
    return int(total)
}
fn total(basket []int) int {
  mut count_by_title := map[int]int{}
  for i := 0; i < basket.len; i++ {
    count_by_title[basket[i]] += 1
  }
  mut counts := count_by_title.values()
  counts.sort()
  n := counts.len
  if n == 5 {
    // there are a steep increase when we get 4 books so the best deal is to get 4 books as much as possible
		// try to redistribute books so group of 4 get maximum length
    delta := counts[2] - counts[1]
    if delta > 0 {
      if counts[0] > delta {
        counts[0] -= delta
        counts[1] += 0
      } else {
        counts[1] += counts[0]
        counts[0] = 0
      }
    }
  }
  return calculate(counts)
}
