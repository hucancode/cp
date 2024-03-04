class Solution {
    fun bagOfTokensScore(tokens: IntArray, power: Int): Int {
        tokens.sort()
        var power = power
        var score = 0
        var i = 0
        var n = tokens.size
        while(i < n) {
            if(power >= tokens[i]) {
                power -= tokens[i]
                score++
            } else if(score > 0) {
                power += tokens[--n] - tokens[i]
            }
            i++
        }
        return score
    }
}
