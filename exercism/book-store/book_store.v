module main
import math

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
  mut counts := [0,0,0,0,0]
  for i := 0; i < basket.len; i++ {
    counts[basket[i]-1] += 1
  }
  counts.sort()
  delta := math.min(counts[0], counts[2] - counts[1])
  counts[0] -= delta
  counts[1] += delta
  return calculate(counts)
}
