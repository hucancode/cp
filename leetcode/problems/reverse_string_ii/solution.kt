class Solution {
    fun reverseStr(s: String, k: Int): String {
        return s.chunkedSequence(k)
            .withIndex()
            .map { (i, arr) -> if(i%2==0) arr.reversed() else arr }
            .joinToString("")
    }
}