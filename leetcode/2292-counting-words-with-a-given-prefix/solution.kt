class Solution {
    fun prefixCount(words: Array<String>, pref: String): Int {
        return words.filter { it.startsWith(pref) }.size
    }
}
