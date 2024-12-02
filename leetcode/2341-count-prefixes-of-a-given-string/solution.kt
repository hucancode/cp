class Solution {
    fun countPrefixes(words: Array<String>, s: String): Int {
        return words.filter { s.startsWith(it) }.size
    }
}
