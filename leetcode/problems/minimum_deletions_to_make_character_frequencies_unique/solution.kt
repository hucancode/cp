class Solution {
    fun minDeletions(s: String): Int {
        val freq = IntArray(26) {0}
        for(c in s) freq[c-'a']++
        freq.sortDescending()
        var cost = 0
        var top = Int.MAX_VALUE
        for(x in freq) {
            val target = Math.max(0, top - 1)
            cost += Math.max(0, x - target)
            top = Math.min(x, target)
        }
        return cost
    }
}