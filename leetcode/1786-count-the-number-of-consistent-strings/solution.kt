class Solution {
    fun countConsistentStrings(allowed: String, words: Array<String>): Int {
        val whiteList = allowed.toSet()
        return words.filter { it.all { whiteList.contains(it) }}
            .count()
    }
}
