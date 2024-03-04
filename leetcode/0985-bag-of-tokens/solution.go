func bagOfTokensScore(tokens []int, power int) int {
    slices.Sort(tokens)
    score := 0
    i := 0
    n := len(tokens)
    for i < n {
        if power >= tokens[i] {
            power -= tokens[i]
            score++
        } else if score > 0 {
            n--
            power += tokens[n] - tokens[i]
        }
        i++
    }
    return score
}
