class Solution {
    fun reverseWords(s: String): String {
        return s.splitToSequence(' ').map { it.reversed() }.joinToString(" ")
    }
}