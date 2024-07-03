class Solution {
    fun threeConsecutiveOdds(arr: IntArray): Boolean {
        return arr.asSequence()
            .windowed(3)
            .any { it.all { it % 2 != 0 } }
    }
}
