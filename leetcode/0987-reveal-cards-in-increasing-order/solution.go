func deckRevealedIncreasing(deck []int) []int {
    slices.Sort(deck)
    ret := make([]int, len(deck))
    q := make([]int, len(deck))
    for i := range q {
        q[i] = i
    }
    take := true
    for len(q) != 0 {
        i := q[0]
        q = q[1:]
        if take {
            x := deck[0]
            deck = deck[1:]
            ret[i] = x
        } else {
            q = append(q, i)
        }
        take = !take
    }
    return ret
}
