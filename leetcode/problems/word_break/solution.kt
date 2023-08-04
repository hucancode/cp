class Solution {
    fun wordBreak(s: String, wordDict: List<String>): Boolean {
        val n = s.length
        val f = Array(n+1) {false}
        f[0] = true
        for(i in 1..n) {
            f[i] = wordDict
                .filter { it.length <= i }
                .any {
                    val j = i - it.length
                    f[j] and (s.substring(j, i) == it) 
                }
        }
        return f[n]
    }
}