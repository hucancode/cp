class Solution {
    fun minExtraChar(s: String, dictionary: Array<String>): Int {
        val pool = dictionary.toSet()
        val n = s.length
        val f = IntArray(n+1) {0}
        for(i in 1..n) {
            f[i] = f[i-1]+1
            for(j in 1..i) {
                val sub = s.substring(j-1,i)
                if(s.substring(j-1,i) !in pool) {
                    continue
                }
                for(k in 0 until j) {
                    f[i] = Math.min(f[i], f[k]+j-k-1)
                }
            }
        }
        return (1..n).map {f[it] + n-it}.min()
    }
}